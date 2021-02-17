use std::env::args;
use std::process::exit;

const PI_SUBST: [u8; 256] = [
    1, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6,
    19, 98, 167, 5, 243, 192, 199, 115, 140, 152, 147, 43, 217, 188,
    76, 130, 202, 30, 155, 87, 60, 253, 212, 224, 22, 103, 66, 111, 24,
    138, 23, 229, 18, 190, 78, 196, 214, 218, 158, 222, 73, 160, 251,
    245, 142, 187, 47, 238, 122, 169, 104, 121, 145, 21, 178, 7, 63,
    148, 194, 16, 137, 11, 34, 95, 33, 128, 127, 93, 154, 90, 144, 50,
    39, 53, 62, 204, 231, 191, 247, 151, 3, 255, 25, 48, 179, 72, 165,
    181, 209, 215, 94, 146, 42, 172, 86, 170, 198, 79, 184, 56, 210,
    150, 164, 125, 182, 118, 252, 107, 226, 156, 116, 4, 241, 69, 157,
    112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230, 45, 168, 2, 27,
    96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15,
    85, 71, 163, 35, 221, 81, 175, 58, 195, 92, 249, 206, 186, 197,
    234, 38, 44, 83, 13, 110, 133, 40, 132, 9, 211, 223, 205, 244, 65,
    129, 77, 82, 106, 220, 55, 200, 108, 193, 171, 250, 36, 225, 123,
    8, 12, 189, 177, 74, 120, 136, 149, 139, 227, 99, 232, 109, 233,
    203, 213, 254, 59, 0, 29, 57, 242, 239, 183, 14, 102, 88, 208, 228,
    166, 119, 114, 248, 235, 117, 75, 10, 49, 68, 80, 180, 143, 237,
    31, 26, 219, 153, 141, 51, 159, 17, 131, 20
];

fn main() {
    let args: Vec<String> = args().collect();
    // collect command-line arguments into a vector
    if args.len() < 3 {
        exit(1);
    }
    // exit if number of arguments is less than 3

    let f_option = args[1] == "f";
    let s_option = args[1] == "s";

    if !f_option && !s_option {
        println!("Invalid switch: {}", args[1]);
        exit(1);
    }

    let l = pad_to_16("hello".to_string().into_bytes());
    println!("{:?}", l);

    let l = append_checksum(l);
    println!("{:?}", l);
}

fn pad_to_16(s: Vec<u8>) -> Vec<u8> {
    let mut output_vec = s.clone();
    let s_len = s.len();
    let padding_len = 16 - (s_len % 16);

    for i in 0..padding_len {
        output_vec.push(padding_len as u8)
    }
    output_vec
}

fn append_checksum(m: Vec<u8>) -> Vec<u8> {
    // C[i] represents ith element of checksum
    // S[i] represents ith element of PI_SUBST
    // L represents l
    // N represents number of bytes of m

    // For i = 0 to 15 do
    //  Set C[i] to 0
    // end
    let mut checksum = vec![0 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let n = m.len();
    // Set L to 0.
    let mut l = 0;
    let mut output_vec = m.clone();

    // For i = 0 to N/16 - 1 do
    for i in 0..n/16 {
        // For j = 0 to 15 do
        for j in 0..16 {
            // Set c to M[i*16 + j]
            let c = m[(i*16) + j];
            // Set C[j] to S[c xor l]
            checksum[j] = PI_SUBST[c as usize ^ l as usize];
            // Set L to C[j]
            l = checksum[j];
        }
    }
    output_vec.append(&mut checksum);
    output_vec
}
