fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

	let mut counter = 0;
	for day in days{
		println!("On the {day} day of Christmas, my true love sent to me");
		if counter > 10 {
			println!("Twelve drummers drumming");
		}
		if counter > 9 {
			println!("Eleven pipers piping");
		}
		if counter > 8 {
			println!("Ten lords a-leaping");
		}
		if counter > 7 {
			println!("Nine ladies dancing");
		}
		if counter > 6 {
			println!("Eight maids a-milking");
		}
		if counter > 5 {
			println!("Seven swans a-swimming");
		}
		if counter > 4 {
			println!("Six geese a-laying");
		}
		if counter > 3 {
			println!("Five golden rings");
		}
		if counter > 2 {
			println!("Four calling birds");
		}
		if counter > 1 {
			println!("Three french hens");
		}
		if counter > 0 {
			println!("Two turtle doves and");
		}
		println!("A partridge in a pear tree\n");
		counter += 1;
	}
}
