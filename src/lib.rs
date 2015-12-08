const CHARMAP: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48,
    0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
    0x59, 0x5A, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E,
    0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7A, 0x30, 0x31, 0x32, 0x33,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2B, 0x2F
];

#[test]
fn it_works() {
    print!("\n");

    let string = "Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure. ";
    let bytes = string.as_bytes();
    let mut out: Vec<u8> = Vec::new();
    for b in bytes.chunks(3) {
        // print!("{:?}\n", c);
        let word: u32 =
            ((b[0] as u32) << 16) |
            ((b[1] as u32) << 8) |
            (b[2] as u32);
        // word = word
        // println!("{:32b}", word);
        let mut bc: [u8; 4] = [0; 4];
        bc[0] = (word >> 18) as u8;
        bc[1] = (word << 14 >> 26) as u8;
        bc[2] = (word << 20 >> 26) as u8;
        bc[3] = (word << 26 >> 26) as u8;
        // println!("{:?}", b);
        out.push(bc[0]);
        out.push(bc[1]);
        out.push(bc[2]);
        out.push(bc[3]);
    }
    print!("\n");
    println!("{:?}", out);
    let s: String = out.iter().map(|c| CHARMAP[*c as usize] as char).collect();
    println!("{}", s);

    print!("\n");

    assert!(false);
}
