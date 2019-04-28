use std;
use std::io::BufRead;
use service;
use configuration;

pub struct Application {

}

impl Application {

	pub fn new() -> Application {
		let app = Application {};
		return app;
	}

	pub fn run(&self) {

		// ===== CONFIGURATION =====
		let conf = configuration::ConfigurationSettings::new();
		if conf.is_none() {
			println!("[FATAL] Configuration error! Cannot start application.");
			return;
		}

		let conf = conf.unwrap();
		println!("[TRACE] get_yaml() returned {:?}.", conf.get_yaml());

		// ===== READING DATA FILE & REGISTRATION =====
		let path = std::path::Path::new("data/mail.tsv");

		let result = std::fs::File::open(path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] {}", error);
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
