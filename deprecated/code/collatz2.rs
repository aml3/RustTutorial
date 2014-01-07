fn main()
{
	let steps = collatz(10, 0);
	print(format!("{:?}\n", steps));

	// print Collazt numbers until we get one with more than 10 steps
	let max_steps = 10;
	let mut i = 2;
	while true
	{
		let steps = collatz(i, 0);
		print(format!("{:?} has {:?} steps\n", i, steps));

		if steps > 10 { break; }

		i += 1;
	}
}

fn collatz(x: int, steps: int) -> int
{
	if x == 1
	{
		return steps;
	}

	match x%2	
	{
		0 => { return collatz(x/2, steps+1); }
		_ => { return collatz(x*3+1, steps+1); }
	}
}
