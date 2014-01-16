use std::os;

fn main() {
	if os::args().len() < 2 {
		println("Error: Please provide a number as argument.");
		return;
	}

	let i = from_str::<int>(os::args()[1]).unwrap();
	println!("{:d} has {:d} Collatz steps", i, collatz(i));
}

fn collatz(N: int) -> int {
	if N == 1 { return 0; }
	match N % 2 {
		0 => { 1 + collatz(N/2) }
		_ => { 1 + collatz(N*3+1) }
	}
}