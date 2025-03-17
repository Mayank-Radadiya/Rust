// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html

fn main() {
    // two data type subsets: scalar and compound.
    let a: i32 = 10;
    println!("{} ", a);

    let b: u32 = 50; // u32 only stored positive number. U means Unsigned.
    let c: i32 = -20; // i32 can store +,_ both. i means signed.
    println!("{}, {}  ", b, c);

    // let a: i8 = -10;  range=> -128 to 127
    // let a: u8 = 20;   range=>  0 to 255

    // other way to write a datatype.
    let x = 1u8; // it assign  1 as u8.
    let y = 20_i8; // it assign 20 as i8.
    let z = 10_00_00; // 100000. "_" is not count in rust.
    println!("{} , {}, {} ", x, y, z);

    let decimal = 98_222; // 98222
    let hax = 0xff; // 255
    let octal = 0o77; // 63
    let binary = 0b1111_0000; // 240
    let byte = b'A'; // 65
                     // 0x = hex, 0o = octal, 0b = binary b = byte
    println!("{} , {}, {}, {}, {} ", decimal, hax, octal, binary, byte);
}
