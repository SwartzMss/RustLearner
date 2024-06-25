
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// 通过泛型实现静态分派
fn static_dispatch<T: Foo>(item: T) {
    println!("{}", item.method());
}

// 通过特征对象实现动态分派
// 动态分派则是在运行时通过特征对象确定实际调用的方法。这使得代码能够在运行时处理多种不同类型的数据，但通常以牺牲一些性能为代价。
fn dynamic_dispatch(item: &dyn Foo) {
    println!("{}", item.method());
}
// 对象安全
// 一个特征能变成特征对象，首先该特征必须是对象安全的，即该特征的所有方法都必须拥有以下特点：

// 返回类型不能是 Self.
// 不能使用泛型参数

fn main() {

    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");

    // 填空
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!")
}   


// 实现以下函数
fn hatch_a_bird(random_number: u8) -> Box<dyn Bird> {
    if random_number == 2 {
        Box::new(Duck)
    } else {
        Box::new(Swan)
    }
}