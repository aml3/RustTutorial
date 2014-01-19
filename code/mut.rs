fn main() {
    let mut x = ~10;
    println(fmt!("*x = %?", *x));
//    x = ~20;
    println(fmt!("*x = %?", *x));
    *x = 30;
    println(fmt!("*x = %?", *x));
}
