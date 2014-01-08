fn main() {
	let mut i = 1;

	// Basic function call
	println("Single Call");
	let steps = collatz(i);
	println(format!("{:?} has {:?} steps", i, steps));

	// Loop until we find a number with more than 10 steps
	println("\nUsing `loop`");
	loop { // equivalent to `while true`
		let steps = collatz(i);
		println(format!("{:?} has {:?} steps", i, steps));

		if steps > 10 { break; }

		i += 1;
	}

	// Find the first 10 Collatz numbers
	println("\nUsing `for`");
	for i in range(1, 11) {
		let steps = collatz(i);
		println(format!("{:?} has {:?} steps", i, steps));
	}

	// Equivalent loop
	println("\nUsing `while`");
	i = 1;
	while i <= 10 {
		let steps = collatz(i);
		println(format!("{:?} has {:?} steps", i, steps));
		
		i += 1;
	}
}

fn collatz(N: int) -> int {
	return collatz2(N, 0);
}

fn collatz2(N: int, steps: int) -> int {
	if N == 1 { return steps; }

	match N%2 {
		0 => { return collatz2(N/2, steps+1); }
		_ => { return collatz2(N*3+1, steps+1); }
	}
}
