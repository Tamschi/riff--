#![doc(html_root_url = "https://docs.rs/riff--/0.0.1")]
#![warn(clippy::pedantic)]

use std::{
	convert::TryInto,
	env::args_os,
	ffi::OsStr,
	fs::{read, OpenOptions},
	io::Write,
};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

#[allow(clippy::missing_panics_doc)]
pub fn main() {
	for arg in args_os().skip(1) {
		let data = read(arg.as_os_str()).unwrap();
		let mut data = data.as_slice();

		let bin_size = data.len();

		while !data.is_empty() {
			if !data.starts_with(b"RIFF") {
				data = &data[1..];
				continue;
			}

			let len: usize = u32::from_le_bytes([data[4], data[5], data[6], data[7]])
				.try_into()
				.expect("This is a < 32bit system, not supported.");

			if len > data.len() {
				eprintln!("len {} > data.len {}", len, data.len());
				data = &data[4..];
				continue;
			}

			let mut path = arg.clone();
			path.push(OsStr::new(&format!(".{:#x}.wav", bin_size - data.len())));

			OpenOptions::new()
				.create_new(true)
				.write(true)
				.open(path)
				.unwrap()
				.write_all(&data[..len])
				.unwrap();

			data = &data[len..];
		}
	}
}
