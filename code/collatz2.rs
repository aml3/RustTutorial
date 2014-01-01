fn main()
{
	let steps = collatz(10, 0);
	print(format!("{:?}\n", steps));
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
