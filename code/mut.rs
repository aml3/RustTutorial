fn main() {
    let mut x = Box::new(10);
    println!("*x = {}", *x);
    
    //    x = Box::new(20);
    println!("*x = {}", *x);

    *x = 30;
    println!("*x = {}", *x);
}
