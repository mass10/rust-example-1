extern crate sqlite;

pub struct Service {

	_connection: Option<sqlite::Connection>,
}

impl Service {

	pub fn init(&mut self) {

		let connection = self.open();
		let sql = r"
		CREATE TABLE USERS(MAIL NVARCHAR(999) NOT NULL, NAME NVARCHAR(999) NOT NULL)
		";
		let result = connection.execute(sql);
		if result.is_err() {
			let error = result.err().unwrap();
			println!("[ERROR] can't insert record. reason: {}", error);
			return;
		}
		println!("[INFO] Ok.");
	}

	pub fn register(&mut self, mail: String, name: String) {

		let connection = self.open();
		let sql = r"
		INSERT INTO USERS(MAIL, NAME) VALUES(?, ?)
		";
		let result = connection.prepare(sql);
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

	fn open(&mut self) -> &sqlite::Connection {

		if self._connection.is_some() {
			return self._connection.as_mut().unwrap();
		}
		self._connection = Some(sqlite::open(":memory:").unwrap());
		let connection = self._connection.as_mut().unwrap();
		return connection;
	}

	pub fn dump(&mut self) {

		let connection = self.open();
		let sql = "SELECT MAIL, NAME FROM USERS";
		let result = connection.prepare(sql);
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

	pub fn new() -> Service {
		let s = Service { _connection: None };
		return s;
	}
}
