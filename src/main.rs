// ★全部ここに書かなきゃダメ
mod application;
mod service;
mod configuration;

fn main() {

	let app = application::Application::new();
	app.run();
}
