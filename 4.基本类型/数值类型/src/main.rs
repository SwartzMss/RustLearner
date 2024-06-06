use std::ops::{Range, RangeInclusive};

fn main() {
    println!("Hello, world!");
    //如果我们没有显式的给予变量一个类型，那编译器会自动帮我们推导一个类型
    // 移除某个部分让代码工作
    // let x: i32 = 5;
    // let mut y: u32  = 5;
    // y = x as u32;
    // let z = 10; // 这里 z 的类型是? 

    // 这个代码的意思是将一个 u8 类型的值 38 转换为 u16 类型，并将其赋值给变量 v
    let _v: u16 = 38_u8 as u16;

    // 解决代码中的错误和 `panic`
    // let v1 = 251_u8 + 8;
    // let v1 = 251_u8.wrapping_add(8);
    // let v2 = i8::checked_add(251, 8).unwrap();
    // 使用 checked_add 来安全地处理 i8 溢出，并处理 Option 的返回值
    // unwrap_or_else 方法用于处理 Option 类型的值。如果 Option 是 Some，它会返回其中的值。
    // 如果是 None，它会调用提供的闭包，并返回闭包的结果。
    // let v2 = i8::checked_add(120, 7).unwrap_or_else(|| {
    //     println!("Overflow occurred");
    //     0 // 或者你希望的其他值
    // });
    // println!("{},{}",v1,v2);

    //  两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
    // let mut sum = 0;
    // for i in -3..3 {
    //     println!("sum = {}, i = {}",sum, i);
    //     sum += i
    // }
    // println!("sum = {}",sum);
    // assert!(sum == -3);

    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

}
