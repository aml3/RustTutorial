use std::iter;

fn main()
{
	let mut is_open: ~[bool] = ~[false, ..100];

	for pass in iter::range_inclusive(1, 100)
	{
		for door in iter::range_step_inclusive(pass, 100, pass)
		{
			is_open[door-1] = !is_open[door-1];
		}
	}

	for door in iter::range_inclusive(1, 100)
	{
		if is_open[door-1]
		{
			println(format!("Door {:?} is open.", door));
		}
	}
}
