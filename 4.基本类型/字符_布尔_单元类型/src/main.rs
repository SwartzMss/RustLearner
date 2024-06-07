
use std::mem::size_of_val;


fn print_char(c : char) {
    println!("{}", c);
}
fn main() {
    println!("Hello, world!");

    // 在 Rust 中，字符类型 char 总是占用 4 个字节（32 位），因为 char 表示一个 Unicode 标量值，并且 Unicode 标量值的最大长度是 4 个字节。
    // 因此，使用 size_of_val 来获取 char 的大小时，结果应该是 4 而不是 1 或 3。
    // let c1: char = 'a';
    // assert_eq!(size_of_val(&c1),4); 
    // let c2 = '中';
    // assert_eq!(size_of_val(&c2),4); 

    // let c = '中';
    // print_char(c);

    // let _f: bool = false;
    // let t = false;
    // if !t {
    //     println!("Success!")
    // }

    // let unit: () = ();
    // assert!(size_of_val(&unit) == 0);

    
}
