extern crate yaml_rust;

use std;
use std::io::Read;

pub struct ConfigurationSettings {

	_conf: Option<yaml_rust::yaml::Yaml>,
	_conf_vec: Vec<yaml_rust::yaml::Yaml>,
}

impl ConfigurationSettings {

	pub fn new() -> Option<ConfigurationSettings> {

		let mut conf = ConfigurationSettings {
			_conf: None,
			_conf_vec: Vec::new(),
		};
		if !conf.configure() {
			return None;
		}
		return Some(conf);
	}

	fn read_text_file(path: String) -> String {

		// open file
		let path = std::path::Path::new(path.as_str());
		let result = std::fs::File::open(path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[TRACE] {}", error);
			return String::new();
		}

		// read to buffer
		let mut f = result.unwrap();
		let mut buf = String::new();
		f.read_to_string(&mut buf).unwrap();

		return buf;
	}

	fn load_yaml(path: String) -> Option<std::vec::Vec<yaml_rust::yaml::Yaml>> {

		// read conf
		let buf = ConfigurationSettings::read_text_file(path);

		// parse as Yaml
		let docs = yaml_rust::yaml::YamlLoader::load_from_str(buf.as_str());
		if docs.is_err() {
			let error = docs.err().unwrap();
			println!("[TRACE] {}", error);
			return None;
		}
		let docs = docs.unwrap();

		return Some(docs);
	}

	fn configure(&mut self) -> bool {

		// open
		let path = String::from("conf/settings.yaml");
		let docs = ConfigurationSettings::load_yaml(path);

		let docs = docs.unwrap();
		// println!("[TRACE] {:?}", docs);
		self._conf_vec = docs;
		return true;
	}

	pub fn get_yaml(self) -> yaml_rust::yaml::Yaml {

		if self._conf.is_none() {
			return yaml_rust::yaml::Yaml::from_str("");
		}
		return self._conf.unwrap();
	}
}
