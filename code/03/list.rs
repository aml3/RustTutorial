type List = Option<~Node>;

struct Node {
   val: int,
   tail: List
}

fn construct_list(n: int, x: int) -> List {
    if n == 0 {
        None
    } else {
        Some(~Node{val: x, tail: construct_list(n - 1, x + 1)})
    }
}

fn main() {
    let node1 = Node {val:10,  tail: None};
    let mut node2 = Node {val: 10, tail: None};
    node2.val = 15; 
        // node1.val = 15; 
    let l10 = construct_list(5, 10);
}
