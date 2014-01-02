fn main()
{
	let x = get_hello();
	println(x);

	let x = get_hello2();
	println(x);
}

fn get_hello() -> &str
{
	return "Hello World!";
}

// lines without a `;` are the same as using a return statement
// So `"Hello World!"` is the same as the above
fn get_hello2() -> &str
{
	"Hello World!"
}
