use std::io::Write;
use std::fs;

fn main() {
    let mut f = fs::File::create("memory/rom.mmap").expect("Could not open file");
    f.write_all(b"!7:16!1:2!").expect("Unable to write file");
    f.write_all(&[0,0x00, 0xff,1, 0x0f, 0x12]).expect("Unable to write file");;
    f.flush().expect("Unable to flush file");
}