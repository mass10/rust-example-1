extern crate yaml_rust;

use std;
use std::io::Read;
// use yaml_rust::yaml::Yaml;

pub struct ConfigurationSettings {

	_conf: Option<yaml_rust::yaml::Yaml>,
}

impl ConfigurationSettings {

	pub fn new() -> Option<ConfigurationSettings> {

		let conf = ConfigurationSettings {
			_conf: None
		};
		if !conf.configure() {
			return None;
		}
		return Some(conf);
	}

	fn configure(&self) -> bool {

		// ===== OPEN =====
		let path = std::path::Path::new("conf/settings.yaml");

		let result = std::fs::File::open(path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("{}", error);
			return false;
		}

		// ===== CONFIGURE =====
		let mut f = result.unwrap();
		let mut buf = String::new();
		f.read_to_string(&mut buf).unwrap();
		let docs = yaml_rust::yaml::YamlLoader::load_from_str(buf.as_str());
		if docs.is_err() {
			let error = docs.unwrap();
			println!("{:?}", error);
			return false;
		}
		let docs = docs.unwrap();
		let doc = &docs[0];
		// self._conf = Some(*doc);
		println!("{:?}", doc);
		return true;
	}
}