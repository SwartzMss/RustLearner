
/* 使用两种方法填空 */
fn main() {
    let v:&'static str = "hello";
    // let v: &'static str = Box::leak(Box::new(String::from("hello")).into_boxed_str());
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}


// 生命周期标注在模板中的用法
// 在模板（泛型）中，生命周期标注通常用于指明类型中的引用必须比某个生命周期活得更久。
// struct Ref<'a, T: 'a>(&'a T);
// struct RefPair<'a, 'b, T: 'a + 'b> {
//     first: &'a T,
//     second: &'b T,
// }

// 生命周期标注在结构体中的用法
// 结构体中使用生命周期标注，表示结构体包含的引用的生命周期。
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }


// 生命周期标注在特征中的用法
// 在特征中使用生命周期标注，表示实现该特征的类型包含的引用的生命周期。
// trait PrintDebug<'a> {
//     fn print_debug(&self, value: &'a str);
// }


// 生命周期标注在函数中的用法
// 函数中使用生命周期标注，表示函数参数和返回值的引用的生命周期。
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// 生命周期标注在方法中的用法
// 在方法中使用生命周期标注，表示方法参数和返回值的引用的生命周期。.
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
    
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }


// 生命周期省略规则
// Rust 编译器有一些内置的规则，当我们没有显式指定生命周期时，它会自动推断生命周期。这些规则包括：

// 输入生命周期省略规则：

// 每个引用参数都会获得一个独立的生命周期。
// 如果一个方法只有一个输入引用参数，编译器会赋予它一个独立的生命周期。
// 输出生命周期省略规则：

// 如果一个方法的输入引用参数只有一个，编译器会推断输出引用的生命周期与这个输入引用的生命周期相同。
// 如果一个方法有多个输入引用参数且没有指定生命周期，编译器无法推断输出引用的生命周期，因此会报错。