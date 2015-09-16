fn increment_mut(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] = v[i] + 1;
    }
}

fn main() {
   let mut p : Vec<i32> = vec![1, 2, 3];
   increment_mut(&mut p);
   for &x in p.iter() {
      print!("{} ", x);
   }
}
