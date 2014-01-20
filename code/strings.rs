fn main() {
    let string = ~"This is a string";
    let subst1 = string.slice(10, 16).to_owned();
    let subst2 = string.slice_from(10).to_owned();
    println!("{}", std::str::eq(&subst1, &subst2));
    let doublesub = subst1 + subst2;
    println!("{}", doublesub);
    //Using split iterator to print word by word
    for tempstr in string.split(' ') {
        println!("{}", tempstr);
    }
    //Another useful function of the split iterator
    //collect() Creates a vector that = ~["This", "is", "a", "string"]
    let wordvec: ~[&str] = string.split(' ').collect(); 
    for &s in wordvec.iter() {
        println!("{}", s);
    }
}
