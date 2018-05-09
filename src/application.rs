extern crate yaml_rust;

use std;
use std::io::BufRead;
use std::io::Read;
use service;

pub struct Application {

}

impl Application {

	pub fn configure(&self) {

		// ===== OPEN =====
		let path = std::path::Path::new("conf/settings.yaml");

		let result = std::fs::File::open(path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("{}", error);
			return;
		}

		// ===== CONFIGURE =====
		let mut f = result.unwrap();
		let mut buf = String::new();
		f.read_to_string(&mut buf).unwrap();
		let docs = yaml_rust::yaml::YamlLoader::load_from_str(buf.as_str()).unwrap();
		let doc = &docs[0];
		println!("{:?}", doc);
	}

	pub fn run(&self) {

		// ===== CONFIGURATION =====
		self.configure();

		// ===== READING DATA FILE & REGISTRATION =====
		let path = std::path::Path::new("data/mail.tsv");

		let result = std::fs::File::open(path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("{}", error);
			return;
		}

		let f = result.unwrap();
		let mut service = service::Service::new();
		service.init();
		let r = std::io::BufReader::new(f);
		for e in r.lines() {
			let mut line = e.unwrap();
			if line == "" {
				continue;
			}
			let mut fields = line.split_whitespace();
			let mail = fields.next().unwrap_or("");
			if mail == "MAIL" {
				continue;
			}
			if mail == "" {
				continue;
			}
			let name = fields.next().unwrap_or("");
			service.register(String::from(mail), String::from(name));
		}

		// ===== SUMMARY =====
		service.dump();
	}
}
