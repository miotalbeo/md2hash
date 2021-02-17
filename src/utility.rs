use std::fs::File;
use std::io::Read;

mod md2;

pub fn usage_no_args(binary: &String) {
    println!("Usage: {} f/s [filename or string]", binary);
}

pub fn usage_with_switch(binary: &String, switch: &String) {
    if switch == "f" {
        println!("Usage: {} f [filename]", binary);
    } else if switch == "s" {
        println!("Usage: {} s [multiple word string]", binary);
    } else {
        usage_no_args(binary);
    }
}

pub fn hash_file(filename: String) {
    let file_contents = read_file(filename);
    let hashed_contents = md2::hash(file_contents);

    let string = format!("{:02x?}", hashed_contents);
    let string = string.replace("[", "");
    let string = string.replace("]", "");
    let string = string.replace(",", "");
    let string = string.replace(" ", "");

    println!("{}", string);
}

pub fn hash_string(strings: Vec<String>) {
    
}

fn read_file(filename: String) -> Vec<u8> {
    let mut f = File::open().expect("Invalid filename");
    let metadata = fs::metadata(&filename).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");
    buffer
}
