// ★全部ここに書かなきゃダメ
mod application;
mod service;
mod configuration;

fn main() {

	println!("[TRACE] ### BEGIN ###");
	let app = application::Application::new();
	app.run();
	println!("[TRACE] --- END ---");
}
