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
	println!("\n-----  Exercise_6  -----\n");
	exercise_6();
	println!("\n-----  Exercise_7  -----\n");
	exercise_7();
	println!("\n-----  Exercise_8  -----\n");
	exercise_8();
	println!("\n-----  Exercise_9  -----\n");
	exercise_9();
	println!("\n-----  Exercise_10  -----\n");
	exercise_10();
}

fn exercise_1() {
	let number = 7;

	if number < 5 {
		println!("condition was true");
	}
	else {
		println!("condition was false");
	}
}

fn exercise_2() {
	let number = 3;

	if number != 0 {
		println!("number was something other than zero");
	}
}

fn exercise_3() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn exercise_4() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn exercise_5() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn exercise_6() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn exercise_7() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn exercise_8() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn exercise_9() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn exercise_10() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}