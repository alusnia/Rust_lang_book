use std::io;

fn main() {
    println!("#### Temperature convertor ####\n");
	
	loop {
		println!("Please enter temperature in given format: <value><unit>");
		println!("Type \"exit\" to close program");
		
		let mut temperature = String::new();

		io::stdin()
			.read_line(&mut temperature)
			.expect("Failed to read line\nExiting program");

		temperature = temperature.trim().to_string();
		if temperature == "exit" {
			break;
		}
		let len = temperature.len();
		if temperature.chars().nth(len - 1) == Some('C') {
			temperature.truncate(len - 1);
			convert(temperature.trim().parse().expect("Error - wrong value"), 'C');
		}
		else if temperature.chars().nth(len - 1) == Some('F') {
			temperature.truncate(len - 1);
			convert(temperature.trim().parse().expect("Error - wrong value"), 'F');
		}
		else {
			println!("\nError - wrong input!");
			println!("Please use only C or F letter\n");
		}
	}
}

fn convert(value: i32, unit: char) {
	
	

	if unit == 'C' {
		let converted_value = (value * 9 / 5) + 32;
		println!("\n{value}{unit} is {converted_value}F\n");
	}
	else {
		let converted_value = (value -32) * 5 / 9;
		println!("\n{value}{unit} is {converted_value}C\n");
	}
}