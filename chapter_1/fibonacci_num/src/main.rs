use std::io;

fn main() {

    println!("Generate n'th Fibonacci number!\n");

	loop {
		println!("Please enter n");
		println!("Type \"exit\" to close program");
		
		let mut number = String::new();

		io::stdin()
			.read_line(&mut number)
			.expect("Failed to read line\nExiting program");

		number = number.trim().to_string();
		if number == "exit" {
			break;
		}
		match number.parse() {
			Ok(num) => {
				if num > 185
				{
					println!("Please enter number in range from 1 to 185!\n");
					0
				}
				else
				{
					fibonacci(num)
				}
			}
			Err(_) => {
				println!("Please enter number in range from 1 to 185!\n");
				continue;
			}
		};
	}
}

fn fibonacci(index: u32) -> u128 {
	let mut value = 1;
	let mut prev_value = 0;
	let mut counter :u32 = 0;

	if index == 0 {
		println!("Please enter positive number!\n");
		return 0;
	}
	while index > counter {
		value = value + prev_value;
		prev_value = value - prev_value;
		counter += 1;
	}
	println!("The {index} number of Fibonacci sequence is: {value}\n");
	return value;
}