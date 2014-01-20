use std::io::buffered::BufferedReader;
use std::io::{stdin,File};
use std::rand::random;

fn main(){
	let mut stdin = BufferedReader::new(stdin());
	let lines: ~[~str]= stdin.lines().collect();
	encrypt(lines, "key", "encrypted");
	decrypt("key", "encrypted", "decrypted");
}

fn encrypt(lines: &[~str], key_name: &str, encrypted_name: &str) {
	let mut input_bytes: ~[u8] = ~[];
	for str_ptr in lines.iter() {
		for byte in (*str_ptr).bytes() {
			input_bytes.push(byte);
		}
	}
	
	let key_file = File::create(&Path::new(key_name));
	let encrypted_file = File::create(&Path::new(encrypted_name));

	let mut random_bytes: ~[u8] = ~[];
	for _ in range(0, input_bytes.len()) {
		let random_byte = random();
		random_bytes.push(random_byte);
	}

	let encrypted_bytes = xor(input_bytes, random_bytes);

	match (key_file, encrypted_file) {
		(Some(mut key), Some(mut encrypted)) => {
				key.write(random_bytes);
				encrypted.write(encrypted_bytes);
			}
		(_, _) => fail!("Error(1) creating files.")
	}
}

fn decrypt(key_name: &str, encrypted_name: &str, decrypted_name: &str) {
	let mut path = Path::new(key_name);
	let key_file = File::open(&path);
	
	path = Path::new(encrypted_name);
	let encrypted_file = File::open(&path);
	
	let mut key_bytes: ~[u8];
	let mut encrypted_bytes: ~[u8];

	match (key_file, encrypted_file) {
		(Some(mut key), Some(mut encrypted)) => {
			key_bytes = key.read_to_end();
			encrypted_bytes = encrypted.read_to_end();
		}
		(_, _) => fail!("Error(2) opening files.")
	}
	
	let decrypted_bytes = xor(key_bytes, encrypted_bytes);

	path = Path::new(decrypted_name);
	let decrypted_file = File::create(&path);
	match decrypted_file {
		Some(mut file) => file.write(decrypted_bytes),
		None => fail!("Error(3) opening file.")
	}
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
	let mut ret = ~[];
	for i in range(0, a.len()) {
		ret.push(a[i] ^ b[i]);
	}
	ret
}
