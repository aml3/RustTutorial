fn borrow2(r : &int) -> int {
   borrow(r)
}

fn borrow(r : &int) -> int {
    *r
}

fn increment(r : &mut int) {
    *r = *r + 1;
}

fn main() {
    let mut x = ~10;
    println!("borrow(x): {:d}", borrow(x));
    increment(x);
    println!("borrow2(x): {:d}", borrow2(x));

    let mut val1 = 10;
    let mut val2 = 20;
    let mut borrowed = &val1;
    borrowed = &val2;
    *borrowed = 11;
}
