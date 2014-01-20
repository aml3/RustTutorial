fn incrementMut(v: &mut [int]) {
    for i in range(0, v.len()) {
        v[i] = v[i] + 1;
    }
}

fn main() {
   let mut p = ~[1, 2, 3];
   incrementMut(p);
   for &x in p.iter() {
      print!("{:d} ", x);
   }
}
