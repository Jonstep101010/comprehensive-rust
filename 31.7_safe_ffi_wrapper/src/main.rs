// TODO: remove this when you're done with your implementation.
#![allow(clippy::upper_case_acronyms)]

mod ffi {
	use std::os::raw::{c_char, c_int};
	#[cfg(not(target_os = "macos"))]
	use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

	// Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
	#[repr(C)]
	pub struct DIR {
		_data: [u8; 0],
		_marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
	}

	// Layout according to the Linux man page for readdir(3), where ino_t and
	// off_t are resolved according to the definitions in
	// /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
	#[cfg(not(target_os = "macos"))]
	#[repr(C)]
	pub struct dirent {
		pub d_ino: c_ulong,
		pub d_off: c_long,
		pub d_reclen: c_ushort,
		pub d_type: c_uchar,
		pub d_name: [c_char; 256],
	}

	// Layout according to the macOS man page for dir(5).
	#[cfg(target_os = "macos")]
	#[repr(C)]
	pub struct dirent {
		pub d_fileno: u64,
		pub d_seekoff: u64,
		pub d_reclen: u16,
		pub d_namlen: u16,
		pub d_type: u8,
		pub d_name: [c_char; 1024],
	}

	unsafe extern "C" {
		pub unsafe fn opendir(s: *const c_char) -> *mut DIR;

		#[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
		pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

		// See https://github.com/rust-lang/libc/issues/414 and the section on
		// _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
		//
		// "Platforms that existed before these updates were available" refers
		// to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
		#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
		#[link_name = "readdir$INODE64"]
		pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

		pub unsafe fn closedir(s: *mut DIR) -> c_int;
	}
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
	path: CString,
	dir: *mut ffi::DIR,
}

impl DirectoryIterator {
	fn new(path: &str) -> Result<DirectoryIterator, String> {
		// Call opendir and return a Ok value if that worked,
		// otherwise return Err with a message.

		// let cstr_path_expect = CString::new(path).expect("valid path from string");
		// more elegant: no panic - provide error in result
		let cstr_path = CString::new(path).map_err(|err| format!("Invalid path: {err}"))?;
		// SAFETY: path.as_ptr() cannot be NULL
		let opendir_ret = unsafe { crate::ffi::opendir(cstr_path.as_ptr()) };
		if opendir_ret.is_null() {
			Err(format!("Could not open {cstr_path:?}"))
		} else {
			Ok(Self {
				path: cstr_path,
				dir: opendir_ret,
			})
		}
	}
}

impl Iterator for DirectoryIterator {
	type Item = OsString;
	fn next(&mut self) -> Option<OsString> {
		// Keep calling readdir until we get a NULL pointer back.
		// SAFETY: self.dir is never NULL.
		let readdir_ret = unsafe { crate::ffi::readdir(self.dir) };
		if readdir_ret.is_null() {
			// reached end of directory (we do not handle errors)
			None
		} else {
			// SAFETY: readdir_ret is not null, d_name is nul-terminated
			let dirname = unsafe { CStr::from_ptr((*readdir_ret).d_name.as_ptr()) };
			let osstr_dirname = OsStr::from_bytes(dirname.to_bytes()).to_os_string();
			Some(osstr_dirname)
		}
	}
}

impl Drop for DirectoryIterator {
	fn drop(&mut self) {
		// Call closedir as needed.
		// SAFETY: self.dir is never NULL
		if unsafe { crate::ffi::closedir(self.dir) } != 0 {
			panic!("closedir failed on: {:?}", self.path)
		}
	}
}

fn main() -> Result<(), String> {
	let iter = DirectoryIterator::new(".")?;
	println!("files: {:#?}", iter.collect::<Vec<_>>());
	Ok(())
}
