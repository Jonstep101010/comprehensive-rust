/*
Let us use our new knowledge to create a multi-threaded link checker.
It should start at a webpage and check that links on the page are valid.
It should recursively check other pages on the same domain and keep doing this until all pages have been validated.

For this, you will need an HTTP client such as reqwest.
You will also need a way to find links, we can use scraper.
Finally, we’ll need some way of handling errors, we will use thiserror.
*/

/*
Tasks:
- Use threads to check the links in parallel: send the URLs to be checked to a channel and let a few threads check the URLs in parallel.
- Extend this to recursively extract links from all pages on the www.google.org domain.
Put an upper limit of 100 pages or so so that you don’t end up being blocked by the site.
*/

use std::{
	collections::HashSet,
	fs::OpenOptions,
	io::Write,
	sync::{Arc, Mutex, mpsc},
};

use reqwest::Url;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::thread;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("request error: {0}")]
	ReqwestError(#[from] reqwest::Error),
	#[error("bad http response: {0}")]
	BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
	url: Url,
	extract_links: bool,
}

// check a specific url
fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
	println!("{:#}", command.url);
	let response = client.get(command.url.clone()).send()?;
	if !response.status().is_success() {
		return Err(Error::BadResponse(response.status().to_string()));
	}

	let mut link_urls = Vec::new();
	if !command.extract_links {
		return Ok(link_urls);
	}

	let base_url = response.url().to_owned();
	let body_text = response.text()?;
	let document = Html::parse_document(&body_text);

	let selector = Selector::parse("a").unwrap();
	let href_values = document
		.select(&selector)
		.filter_map(|element| element.value().attr("href"));
	for href in href_values {
		match base_url.join(href) {
			Ok(link_url) => {
				link_urls.push(link_url);
			}
			Err(err) => {
				println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
			}
		}
	}
	Ok(link_urls)
}
// from solution
type CrawlResult = Result<Vec<Url>, (Url, Error)>;

// mpsc: CrawlCommand
fn main() {
	let start_url = reqwest::Url::parse("https://www.google.org").unwrap();
	let args: Vec<String> = std::env::args().collect();
	let save_file = args.get(1).map(|s| s.as_str());
	if let Some(file) = save_file {
		eprintln!("{file} argument provided!")
	}
	check_sites(start_url, save_file);
}

///
/// runs the loop until no more endpoints remaining
fn worker_crawl_thread(
	command_receiver: Arc<Mutex<std::sync::mpsc::Receiver<CrawlCommand>>>,
	result_sender: mpsc::Sender<CrawlResult>,
) {
	let client = Client::new();
	loop {
		// check endpoints, send result on channel
		let crawl_command = match command_receiver.lock().unwrap().recv() {
			Ok(crawlcommand) => crawlcommand,
			Err(_) => break,
		};
		let crawl_result = match visit_page(
			&client,
			&crawl_command, /* from command_receiver after recv() */
		) {
			Ok(links) => Ok(links),
			Err(err) => Err((crawl_command.url, err)),
		};
		result_sender.send(crawl_result).unwrap();
	}
}
fn spawn_workers(
	command_receiver: mpsc::Receiver<CrawlCommand>,
	result_sender: mpsc::Sender<CrawlResult>,
) {
	// wrap command_receiver in mutex
	let command_receiver_guarded = Arc::new(Mutex::new(command_receiver));
	for _ in 0..NUM_THREADS {
		let command_receiver_guard = command_receiver_guarded.clone();
		let result_sender = result_sender.clone();
		thread::spawn(move || {
			worker_crawl_thread(command_receiver_guard, result_sender);
		});
	}
}

struct CrawlState {
	domain: String,
	visited_sites: std::collections::HashSet<String>,
}

impl CrawlState {
	fn new(start_url: &Url) -> Self {
		let mut new = CrawlState {
			visited_sites: HashSet::new(),
			domain: start_url.domain().unwrap().to_string(),
		};
		new.visited_sites.insert(start_url.to_string());
		new
	}
	///
	/// is domain, has host, not just IP
	fn should_descend_endpoints(&self, url: &Url) -> bool {
		if let Some(url_endpoint) = url.domain() {
			url_endpoint == self.domain
		} else {
			false
		}
	}
	///
	/// not previously encountered
	fn mark_visited(&mut self, url: &Url) -> bool {
		self.visited_sites.insert(url.to_string())
	}
	/// write visited sites to file
	fn filedump(&self, save_file: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
		if let Some(filename) = save_file {
			let mut file = OpenOptions::new()
				.write(true)
				.create(true)
				.truncate(true)
				.open(filename)?;
			for url in &self.visited_sites {
				file.write_fmt(format_args!("{}\n", url))?;
			}
		}
		Ok(())
	}
}

const NUM_THREADS: usize = 16;
///
/// stores crawlstate, updates visited & bad urls
fn monitor_workers(
	start_url: Url,
	command_sender: mpsc::Sender<CrawlCommand>,
	result_receiver: mpsc::Receiver<CrawlResult>,
	save_file: Option<&str>,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
	// initialize crawlstate
	let mut crawl_state = CrawlState::new(&start_url);
	let initial_crawl_command = CrawlCommand {
		url: start_url,
		extract_links: true,
	};
	command_sender.send(initial_crawl_command).unwrap();
	let mut sites_remaining = 1;
	let mut bad_urls: Vec<Url> = vec![];

	while sites_remaining > 0 {
		// receive results
		let crawl_result = result_receiver.recv().unwrap();
		sites_remaining -= 1;
		// match, append and redispatch or error out
		match crawl_result {
			Ok(urls) => {
				for url in urls {
					// check if visited, otherwise mark as visited
					if crawl_state.mark_visited(&url) {
						// determine if we should extract links
						let extract_links = crawl_state.should_descend_endpoints(&url);
						// set up CrawlCommand and send
						command_sender
							.send(CrawlCommand {
								extract_links,
								url: url.clone(),
							})
							.unwrap();
						sites_remaining += 1;
					}
				}
			}
			Err((url, err)) => {
				bad_urls.push(url);
				eprintln!("crawling error: {:#}", err);
				continue;
			}
		}
	}
	if !bad_urls.is_empty() {
		eprintln!("Bad URLs: {:#?}", bad_urls);
	}
	crawl_state.filedump(save_file)
}

// sets up infrastructure for supervising/monitoring as well as dispatching workers
fn check_sites(start_url: Url, save_file: Option<&str>) {
	// from solution: use command_sender, command_receiver, result_sender, result_receiver)
	let (command_sender, command_receiver) = mpsc::channel::<CrawlCommand>();
	let (result_sender, result_receiver) = mpsc::channel::<CrawlResult>();
	spawn_workers(command_receiver, result_sender);
	monitor_workers(start_url, command_sender, result_receiver, save_file).unwrap();
}
