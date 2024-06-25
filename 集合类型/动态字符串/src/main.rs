
// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
fn main() {
    // let mut s: String = String::from("hello, ");
    // s.push_str(&("world".to_string()));
    // s.push('!');

    // move_ownership(&s);

    // assert_eq!(s, "hello, world!");

    // println!("Success!")

    let mut s = String::from("hello, world");

    let slice1: &str = &s; // 使用两种方法
    assert_eq!(slice1, "hello, world");
 
    let slice2: &str = &s[0..5];
    assert_eq!(slice2, "hello");
 
    let slice3: &mut String = &mut s; 
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!");

    let mut s = String::with_capacity(25);

    println!("{}", s.capacity()); // 打印预分配的容量，这里应该是 25

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity()); // 容量不会改变，依然是 25
    }

    println!("Success!");
}

fn move_ownership(s: &String) {
    println!("ownership of \"{}\" is moved here!", s)
}