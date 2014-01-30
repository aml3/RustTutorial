fn count(m: &str, n: int) {
    for i in range(1, n) {
        println!("{:s}{:d}", m, i); 
    }
}

fn main() {
    let mut n = 10;
    spawn(proc() { count("A", n); });
    spawn(proc() { count("B", n); });
    spawn(proc() { count("C", n); });
}
