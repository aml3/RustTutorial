fn main() {
    let string = Box::new("This is a string");

    let subst1 = string[10..16].to_owned().to_string();
    let subst2 = string[10..].to_owned().to_string();
    println!("{}", &subst1==&subst2);

    let doublesub = subst1 + &subst2; // this part is tricky, refer to Rust book for more detail
    println!("{:?}", doublesub);
    
    //Using split iterator to print word by word
    for tempstr in string.split(' ') {
        println!("{}", tempstr);
    }
    
    //Another useful function of the split iterator
    //collect() Creates a vector that = ["This", "is", "a", "string"]
    let wordvec : Vec<&str> = string.split(' ').collect(); 
    for s in wordvec {
        println!("{}", s);
    }
}
