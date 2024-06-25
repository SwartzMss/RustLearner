// 引用参数中的生命周期 'a 至少要跟函数活得一样久
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// 可变引用依然需要标准生命周期
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// 下面代码中，每个参数都拥有自己独立的生命周期，事实上，这个例子足够简单，因此它们应该被标记上相同的生命周期 `'a`，但是对于复杂的例子而言，独立的生命周期标注是可能存在的
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// 返回一个通过参数传入的引用是很常见的，但是这种情况下需要标注上正确的生命周期
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 当结构体包含引用时，必须为这些引用指定生命周期。
// `i32` 的引用必须比 `Borrowed` 活得更久
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 7;
    let y = 9;
    
    print_one(&x);
    print_multi(&x, &y);
    
    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}