use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 2 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let fname = args[1];
        let path = Path::new(fname.clone());
        let msg_file = File::open(&path);

        match (msg_file) {
            Some(mut msg) => {
                let msg_bytes: ~[u8] = msg.read_to_end();
                let share1_file 
                       = File::create(&Path::new(fname + ".share1"));
                let share2_file 
                       = File::create(&Path::new(fname + ".share2"));
                
                match (share1_file, share2_file) {
                    (Some(share1), Some(share2)) => { 
                        split(msg_bytes, share1, share2); 
                        } ,
                    (_, _) => fail!("Error opening output files!"),
                }
            } ,
            None => fail!("Error opening message file: {:s}", fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: &[u8], mut share1: File, mut share2: File) {
    let mut random_bytes: ~[u8] = ~[];
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in range(0, msg_bytes.len()) {
	let random_byte = random();
	random_bytes.push(random_byte);
    }
    
    let encrypted_bytes = xor(msg_bytes, random_bytes);
    share1.write(random_bytes);
    share2.write(encrypted_bytes);
}
