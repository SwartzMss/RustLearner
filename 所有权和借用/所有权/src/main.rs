
fn main() {
    // 使用尽可能多的方法来通过编译
    // let x = String::from("hello, world");
    // let y:&str = &x;
    // println!("{},{}",x,y);

    // let s1 = String::from("hello, world");
    // let s2 = take_ownership(s1);
    // println!("{}", s2);

    // let s = give_ownership();
    // println!("{}", s);

    // let s = String::from("hello, world");
    // print_str(&s);
    // println!("{}", s);

    // let s = String::from("hello, ");
    // // 只修改下面这行代码 !
    // let mut s1 = s;
    // s1.push_str("world")

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
    
}




// // 只能修改下面的代码!
// fn take_ownership(s: String)->String {
//     println!("{}", s);
//     s
// }

// fn print_str(s: &str)  {
//     println!("{}",s)
// }

// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let _s = s.clone().into_bytes();
//     s
// }