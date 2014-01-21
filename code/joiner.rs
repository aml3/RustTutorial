use std::os;
use std::io::File;
use std::io::stdout;
use std::io::buffered::BufferedWriter;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <share1> <share2>", args[0]); 
    } else {
        let path1 = Path::new(args[1].clone());
        let path2 = Path::new(args[2].clone());

        let path1_file = File::open(&path1);
        let path2_file = File::open(&path2);

        match (path1_file, path2_file) {
            (Some(mut share1), Some(mut share2)) => { 
                    let share1_bytes: ~[u8] = share1.read_to_end();
                    let share2_bytes: ~[u8] = share2.read_to_end();

                    if share1_bytes.len() != share2_bytes.len() {
                        fail!("Files are different sizes!");
                    }
                    join(share1_bytes, share2_bytes); 
            } ,
            (_, _) => fail!("Error opening input files!"),
        }
    }
}

fn join(share1_bytes: &[u8], share2_bytes: &[u8]) {
    assert!(share1_bytes.len() == share2_bytes.len());
    let decrypted_bytes = xor(share1_bytes, share2_bytes);
    let mut stdout = BufferedWriter::new(stdout());
    stdout.write(decrypted_bytes);
    stdout.flush();
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}
