use std::io;
use std::cmp::Ordering;

fn exercise_1() {
	let mut x = 5;

	println!("The value of x is: {x}");
	x = 6;
	println!("The value of x is: {x}");
}

fn exercise_2() {
	let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn exercise_3() {
	let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

	println!("The value of x is: {x}");
    println!("The value of y is: {y}");
	println!("The value of z is: {z}");
}

fn exercise_4() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

	println!("The value of five_hundred is: {five_hundred}");
	println!("The value of six_point_four is: {six_point_four}");
	println!("The value of one is: {one}");
}

fn exercise_5() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was negative or not a number");

	match index.cmp(&5) {
		Ordering::Less => {
			let element = a[index];

    		println!("The value of the element at index {index} is: {element}");
		},
		_ => println!("Value must be in range from 0 to 4!"),
	}
	
}

fn main() {
	println!("\n-----  Exercise_1  -----\n");
    exercise_1();
	println!("\n-----  Exercise_2  -----\n");
	exercise_2();
	println!("\n-----  Exercise_3  -----\n");
	exercise_3();
	println!("\n-----  Exercise_4  -----\n");
	exercise_4();
	println!("\n-----  Exercise_5  -----\n");
	exercise_5();
}