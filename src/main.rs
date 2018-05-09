extern crate sqlite;
extern crate yaml_rust;

mod application;
mod service;

fn main() {

	let app = application::Application{};
	app.run();
}
