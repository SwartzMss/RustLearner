/* 填空 */
fn main() {
    println!("{argument}", argument = "test"); // => "test"
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");// => Alice, this is Bob. Bob, this is Alice
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("21{0}{1}", 1, 2), "2112");
    println!("Success!");

    /* 填空 */
    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a}{c}{b}",a = "a", b = 'b', c = 3 ), "a 3 b");
    
    // 下面两个都是通过 5 个空格来填充
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"  
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"
}