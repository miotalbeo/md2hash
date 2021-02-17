mod md2;

fn main() {
    let l = md2::pad_to_16("hello".to_string().into_bytes());
    let l = md2::append_checksum(l);
    let l = md2::calculate_hash(l);

    let l = format!("{:02x?}", l);
    let l = l.replace("[", "");
    let l = l.replace("]", "");
    let l = l.replace(",", "");
    let l = l.replace(" ", "");
    println!("{}", l);
}
