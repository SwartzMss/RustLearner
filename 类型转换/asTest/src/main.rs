fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;


    let c2 = integer as char;

    assert_eq!(integer, 'a' as u8);

    println!("Success!")
}
