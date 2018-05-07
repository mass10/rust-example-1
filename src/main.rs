use std::path::Path;

fn on_entry(e: &Path) -> std::io::Result<()> {

	let path_str = e.as_os_str().to_str().unwrap();
	println!("{}", path_str);
	Ok(())
}

# [allow(unused)]
fn search(e: &Path, handler: &Fn(&Path) -> std::io::Result<()>) -> std::io::Result<()> {

	if !e.exists() {
		println!("[TRACE] invalid path {}", e.to_str().unwrap());
		return Ok(());
	}
	if e.is_dir() {
		let it: std::fs::ReadDir = std::fs::read_dir(e)?;
		for e in it {
			let entry = e.unwrap();
			let path = entry.path();
			search(&path, handler);
		}
	}
	else {
		return handler(e);
	}
	Ok(())
}

fn main() {

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("path?");
		return;
	}
	let path = Path::new(&args[1]);
	let _result = search(path, &on_entry);
}
