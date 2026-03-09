fn main() {
    println!("\n-----  Exercise_1  -----\n");
    exercise_1();
	println!("\n-----  Exercise_2  -----\n");
	exercise_2();
	println!("\n-----  Exercise_3  -----\n");
	exercise_3();
	println!("\n-----  Exercise_4  -----\n");
	exercise_4();
}

fn exercise_1() {
	another_function_1(5);
}

fn another_function_1(x: i32) {
	println!("The value of x is: {x}");
}

fn exercise_2() {
	print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
	println!("The measurement is: {value}{unit_label}");
}

fn exercise_3() {
    let x = five();

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn exercise_4() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}