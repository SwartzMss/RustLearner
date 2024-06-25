// From 特征允许让一个类型定义如何基于另一个类型来创建自己，因此它提供了一个很方便的类型转换的方式。

// From 和 Into 是配对的，我们只要实现了前者，那后者就会自动被实现：
//     只要实现了 impl From<T> for U， 就可以使用以下两个方法: let u: U = U::from(T) 和 let u:U = T.into()，
//     前者由 From 特征提供，而后者由自动实现的 Into 特征提供。

// From 和 Into 是配对的，我们只要实现了前者，那后者就会自动被实现：只要实现了 impl From<T> for U， 
// 就可以使用以下两个方法: let u: U = U::from(T) 和 let u:U = T.into()，前者由 From 特征提供，而后者由自动实现的 Into 特征提供


#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // 实现 `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}
fn main() {
    let my_str = "hello";

    // 以下三个转换都依赖于一个事实：String 实现了 From<&str> 特征
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // 这里需要显式地类型标注
    let string3: String = my_str.into();

    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // 使用两种方式修复错误
    // 1. 哪个类型实现 From 特征 : impl From<char> for i32, 你可以查看一下之前提到的文档，来找到合适的类型
    // 2. 上一章节中介绍过的某个关键字
    let i3: i32 = 'a' as i32; // 使用 as 关键字来转换

    assert_eq!(i3, 'a' as i32);

    // 使用两种方法来解决错误
    let s: String = 'a'.to_string(); // 使用 to_string 方法
    // 或者使用 String::from 方法
    let s2: String = String::from('a');

    assert_eq!(s, "a");
    assert_eq!(s2, "a");

    println!("Success!");

    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!");

    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // 填空
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));


    let n: i16 = 256;

    // Into 特征拥有一个方法`into`,
    // 因此 TryInto 有一个方法是 ?
    // 类似于 From 和 Into, TryFrom 和 TryInto 也是用于类型转换的泛型特征。
    // 但是又与 From/Into 不同, TryFrom 和 TryInto 可以对转换后的失败进行处理，然后返回一个 Result。
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);

    println!("Success!")
}


#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item:i32) -> Number
    {
        Number{value:item}
    }
}


use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}