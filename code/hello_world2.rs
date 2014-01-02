fn main()
{
	let x = get_hello();
	println(x); // prints `Hello World!`

	// variables can be redeclared with the same name
	let x = get_hello2();
	println(x); // prints `Hello World, again!`
}

fn get_hello() -> &str
{
	return "Hello World!";
}

// lines without a `;` are the same as using a return statement
fn get_hello2() -> &str
{
	"Hello World, again!"
}
