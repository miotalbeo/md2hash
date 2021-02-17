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
