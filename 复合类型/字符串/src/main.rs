
// 使用至少两种方法来修复错误
fn main() {
    // let s: Box<str> = "hello, world".into();
    // greetings(&s);

    // let mut s = String::from("hello");
    // s.push(',');
    // s.push_str("world");
    // s += "!".to_string().as_str();

    // println!("{}", s)

    // let s1 = String::from("hello,");
    // let s2 = String::from("world!");
    // let s3 = format!("{}{}", s1, s2); // 使用 format! 宏来连接字符串
    // assert_eq!(s3,"hello,world!");
    // println!("{}",s1);

    // 注意，这并不是 `&str` 类型了！
    let _bytestring: &[u8; 21] = b"this is a byte string";
}

// fn greetings(s: &str) {
//     println!("{}",s)
// }