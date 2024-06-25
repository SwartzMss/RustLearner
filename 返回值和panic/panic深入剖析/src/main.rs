
// Rust 中最简单的错误处理方式就是使用 panic。它会打印出一条错误信息并打印出栈调用情况，最终结束当前线程:

// 若 panic 发生在 main 线程，那程序会随之退出
// 如果是在生成的( spawn )子线程中发生 panic, 那么当前的线程会结束，但是程序依然会继续运行
use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}
// 填空
// 不要修改其它代码
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        std::process::exit(0);
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");

    assert_eq!(add_two("4").unwrap(), 6);
}
