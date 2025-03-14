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

use std::sync::mpsc;

use reqwest::Url;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
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
	println!("Checking {:#}", command.url);
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
	check_sites(start_url);
}

///
/// runs the loop until no more endpoints remaining
fn worker_crawl_thread(
	command_receiver: mpsc::Receiver<CrawlCommand>,
	result_sender: mpsc::Sender<CrawlResult>,
) {
	let client = Client::new();
	loop {
		// check endpoints, send result on channel
		// should have a guard in concurrency?
		let crawl_command = match command_receiver.recv() {
			Ok(crawlcommand) => crawlcommand,
			Err(_) => break,
		};
		let crawl_result = match visit_page(
			&client,
			&crawl_command, /* from command_receiver after recv() */
		) {
			Ok(links) => Ok(links),
			// println!("Links: {links:#?}"),
			Err(err) => Err((crawl_command.url, err)), // println!("Could not extract links: {err:#}"),
		};
		result_sender.send(crawl_result);
	}
}

fn spawn_workers(
	command_receiver: mpsc::Receiver<CrawlCommand>,
	result_sender: mpsc::Sender<CrawlResult>,
) {
}

const NUM_THREADS: usize = 16;
///
/// stores crawlstate, updates visited & bad urls
fn monitor_workers(
	start_url: Url,
	command_sender: mpsc::Sender<CrawlCommand>,
	result_receiver: mpsc::Receiver<CrawlResult>,
) {
	// initialize crawlstate
	let initial_crawl_command = CrawlCommand {
		url: start_url,
		extract_links: true,
	};
	command_sender.send(initial_crawl_command);
	let mut sites_remaining = 1;
	let mut bad_urls: Vec<Url> = vec![];

	while sites_remaining > 0 {
		// receive results
		// match, append and redispatch or error out
	}
	println!("Bad URLs: {:#?}", bad_urls);
}

// sets up infrastructure for supervising/monitoring as well as dispatching workers
fn check_sites(start_url: Url) {
	// from solution: use command_sender, command_receiver, result_sender, result_receiver)
	let (command_sender, command_receiver) = mpsc::channel::<CrawlCommand>();
	let (result_sender, result_receiver) = mpsc::channel::<CrawlResult>();
	spawn_workers(command_receiver, result_sender);
	monitor_workers(start_url, command_sender, result_receiver);
}
