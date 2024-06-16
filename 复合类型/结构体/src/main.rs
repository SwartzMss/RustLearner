// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn check_color(p: Color) {
//     let (x, _, _) = (p.0, p.1, p.2);
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
//  }

//  struct PersonZero {
//     name: String,
//     age: u8,
// }
fn main() {
    // let age = 30;
    // let p = Person {
    //     name: String::from("sunface"),
    //     age,
    //     hobby:String::from("riding")
    // };

    // let u = Unit;
    // do_something_with_unit(u);

    // let  v = Point(0, 127, 255);
    // check_color(Color(v.0, v.1, v.2)); // 转换为 Color 并传递


    // // 你可以在实例化一个结构体时将它整体标记为可变的，但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.
    // let age = 18;
    // let mut p = PersonZero {
    //     name: String::from("sunface"),
    //     age,
    // };

    // // how can you believe sunface is only 18? 
    // p.age = 30;

    // // 填空
    // p.name = String::from("sunfei");


    // 使用结构体字段初始化缩略语法可以减少一些重复代码

    // 结构体的所有权
    // 当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。当这么做时，部分 move 就会发生：变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。

    // 在这种情况下，原变量将无法再被使用，但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。


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


    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }

    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = &f.name;

    // 只能修改这一行
    println!("{}, {}, {:?}",f.name, f.data, f);
} 

// // 填空，让代码工作
// fn do_something_with_unit(u: Unit) {   }

// impl SomeTrait for Unit {  }

// struct Unit;
// trait SomeTrait {
//     // ...定义一些行为
// }