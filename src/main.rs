use std::io;

fn main(){
	let programs = ["main", "hello world"];

	println!("Hi! Welcome to Inaya's Project Selector!");
	println!("Please input your project selection, {}",
		"or type \"list\" to list all available programs!");

	let mut selection = String::new();

	io::stdin()
		.read_line(&mut selection)
		.expect("Oops, something went wrong!");

	if selection.contains("list"){
		println!("Here are the options: ");
		for e in programs {
			println!("{e}");
		}
	}
}
