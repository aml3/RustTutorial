#[deriving(Clone)]
#[deriving(ToStr)]
type LinkedList = Option<~Node>;

#[deriving(Clone)]
struct Node {
   val: int,
   tail: LinkedList
}

trait Length {
   fn length(&self) -> int;
}

#[allow(dead_code)] 
fn length(p: LinkedList) -> int {
   match p {
      None => { 0 }
      Some(node) => { 1 + length(node.tail) }
   }
}

impl Length for LinkedList {
   fn length(&self) -> int {
      match self {
        &Some(ref node) => { 1 + node.tail.length() }
        &None => 0
      }
   }
}

fn construct_list(n: int, x: int) -> LinkedList {
    match n {
        0 => { None }
        _ => { Some(~Node{val: x, tail: construct_list(n - 1, x + 1)}) }
    }
}

fn print_list(p: LinkedList) -> ~str {
    match p {
        None => { ~"" }
        Some(node) => { node.val.to_str() + ", " + print_list(node.tail) }
    }
}

trait Map {
   fn mapr(&mut self, fn(int) -> int);
}


impl Map for LinkedList {
    fn mapr(&mut self, f: fn(int) -> int) {
         match(*self) {
            None => { }
            Some(ref mut current) => { 
               current.val = f(current.val);
               current.tail.mapr(f); 
            }
        } 
    } 
}

fn inc(n: int) -> int { n + 1 }
fn double(n: int) -> int { n * 2 }

fn main() {
    let mut l10 : LinkedList = construct_list(5, 10);
    l10.mapr(inc);
    l10.mapr(double);
    println!("List: {:s}", print_list(l10.clone()));
}
