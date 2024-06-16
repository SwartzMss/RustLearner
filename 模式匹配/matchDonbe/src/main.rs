
// 填空
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South|Direction::North  => { // 在这里匹配 South 或 North
            println!("South or North");
        },
        _ => println!("west"),
    };


    let boolean = true;

    // 使用 match 表达式填空，并满足以下条件
    //
    // boolean = true => binary = 1
    // boolean = false => binary = 0
    let binary = match boolean {
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);

    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // 使用 `matches!` 填空
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }

    let mut count = 0;

    println!("start aa");
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if let  MyEnum::Foo = e { // 修复错误，只能修改本行代码  // 注意 这边的话 得吧匹配的值放前面的
            count += 1;
            println!("aa");
        }
    }

    assert_eq!(count, 2);


    let age = Some(30);
    if let Some(age1) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
       assert_eq!(age1, 30);
    } // 新的 `age` 变量在这里超出作用域
    
    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
}

enum MyEnum {
    Foo,
    Bar
}

fn show_message(msg: Message) {
    match msg {
        Message::Move {x:a,y:b} => { // 这里匹配 Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        __ => println!("no data in these variants")
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}