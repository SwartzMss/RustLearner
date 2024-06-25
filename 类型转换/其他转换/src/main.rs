use std::fmt;
use std::str::FromStr;
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}


// 使用 parse 方法可以将一个 String 转换成 i32 数字，这是因为在标准库中为 i32 类型实现了 FromStr: : impl FromStr for i32
fn main() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!");

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "10".parse().unwrap();
    let from_str: i32 = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}