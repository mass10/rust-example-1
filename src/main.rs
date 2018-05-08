extern crate sqlite;

use std::path::Path;
use std::io::Read;
// use std::io::BufRead;


struct Service {

	_connection: Option<sqlite::Connection>,
}

impl Service {

	fn init(&mut self) {
		let connection = self.open();
		let result = connection.execute("CREATE TABLE USERS(MAIL NVARCHAR(999) NOT NULL, NAME NVARCHAR(999) NOT NULL)");
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] can't insert record. reason: {}", error);
			return;
		}
		println!("[INFO] Ok.");
	}

	fn register(&mut self, mail: String, name: String) {
		let connection = self.open();
		let result = connection.prepare("INSERT INTO USERS(MAIL, NAME) VALUES(?, ?)");
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] can't insert record. reason: {}", error);
			return;			
		}
		let mut statement = result.unwrap();
		statement.bind(1, mail.as_str()).unwrap();
		statement.bind(2, name.as_str()).unwrap();
		statement.next().unwrap();
	}

	fn open(&mut self) -> &mut sqlite::Connection {	
		if self._connection.is_some() {
			return self._connection.as_mut().unwrap();
		}
		self._connection = Some(sqlite::open(":memory:").unwrap());
		let connection = self._connection.as_mut().unwrap();
		return connection;
	}

	fn dump(&mut self) {
		let connection = self.open();
		let result = connection.prepare("SELECT MAIL, NAME FROM USERS");
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] reason: {}", error);
			return;
		}
		let mut statement = result.unwrap();
		while let sqlite::State::Row = statement.next().unwrap() {
			println!("mail={}, name={}",
				statement.read::<String>(0).unwrap(),
				statement.read::<String>(1).unwrap());
		}
	}
}

struct Application {

}

impl Application {

	fn configure(&self) {
		let path = Path::new("data/mail.tsv");
		let mut buf = String::new();
		match std::fs::File::open(path) {
			Err(info) => {
				println!("{}", info);
			},
			Ok(mut f) => {
				f.read_to_string(&mut buf).unwrap();
			}
		};
		println!("{}", buf);
	}

	fn run(&self) {
		self.configure();
		let path = Path::new("data/mail.tsv");
		let result = std::fs::File::open(path);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("{}", error);
			return;
		}
		let f = result.unwrap();
		let mut service = Service { _connection: None };
		service.init();
		let r = std::io::BufReader::new(f);
		use std::io::BufRead;
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
		service.dump();
	}
}

fn main() {

	let app = Application{};
	app.run();
}
