fn main()
{
	let mut is_open: ~[bool] = ~[false, ..100];

	let mut pass = 0;
	let mut door;

	while pass < 100
	{
		door = pass;
		while door < 100
		{
			is_open[door] = !is_open[door];
			door += pass+1;
		}
		pass += 1;
	}

	door = 0;

	while door < 100
	{
		if is_open[door]
		{
			println(format!("Door {:?} is open.", door+1));
		}
		door += 1;
	}
}
