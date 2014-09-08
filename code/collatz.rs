use std::os;

fn main() {
	if os::args().len() < 2 {
		println!("Error: Please provide a number as argument.");
		return;
	}

	let i = from_str::<int>(os::args()[1].as_slice()).unwrap();
	println!("{:d} has {:d} Collatz steps", i, collatz(i));
}

fn collatz(n: int) -> int {
	if n == 1 { return 0; }
	match n % 2 {
		0 => { 1 + collatz(n/2) }
		_ => { 1 + collatz(n*3+1) }
	}
}
