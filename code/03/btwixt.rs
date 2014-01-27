fn twice(f: proc(int) -> int) -> (proc(int) -> int) {
    proc(n: int) { f(f(n)) }
}

fn double(n:int) -> int { n * 2 }

fn main()
{
    let hexaple = twice(twice(double));
    println!("Result: {:d}", hexaple(2));
}
