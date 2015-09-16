fn borrow2(r : &i32) -> i32 {
   borrow(r)
}

fn borrow(r : &i32) -> i32 {
    *r
}

fn increment(r :&mut i32) {
    *r = *r + 1;
}

fn main() {
    let mut x = Box::new(10);
    println!("borrow(x): {}", borrow(&x));
    increment(&mut x);
    println!("borrow2(x): {}", borrow2(&x));

    let mut val1 = 10;
    let mut val2 = 20;
    let mut borrowed = &mut val1;
    borrowed = &mut val2;
    *borrowed = 11;
}
