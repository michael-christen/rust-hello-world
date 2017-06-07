extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
	let secret_number = rand::thread_rng().gen_range(1, 101);
	println!("Secret number: {}", secret_number);

	loop {
		println!("Give me some input!");
		let mut input = String::new();
		io::stdin().read_line(&mut input)
			.expect("Failed to read line");  // What to print on failure
		let input: u32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_)  => continue,
		};
		println!("Input was {}", input);

		match input.cmp(&secret_number) {
			Ordering::Less    => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal   => {
				println!("You win!");
				break;
			}
		}
	}
}
