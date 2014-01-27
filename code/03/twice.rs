//fn twice(n: int, f: extern fn(int) -> int) -> int {
fn twice(n: int, f: |int| -> int) -> int {
    f(f(n))
}

fn successor(n: int) -> int { n + 1 }
fn double(n:int) -> int { n * 2 }

fn main()
{
    println!("Result: {:d}", twice(twice(1, successor), double));
}
