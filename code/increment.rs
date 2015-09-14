fn increment(v: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];

    for x in v.iter() {
        res.push(*x + 1);
    }

    res
}

fn main() {
   let p : Vec<i32> = vec![1, 2, 3];
   let q = increment(p);
   for &x in q.iter() {
      print!("{} ", x);
   }
}
