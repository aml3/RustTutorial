fn count(m: &str, n: int) {
    for i in range(1, n) {
        println!("{:s}{:d}", m, i); 
    }
}

fn main() {
    spawn(proc() { count("A", 1000); });
    spawn(proc() { count("B", 1000); });
    spawn(proc() { count("C", 1000); });
}
