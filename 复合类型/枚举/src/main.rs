
// 修复错误
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C语言风格的枚举定义
// 在 Rust 中，枚举的数值只能是整数类型。Rust 不支持枚举值使用浮点数类型。所有枚举变体必须是整数类型，包括它们的显式值。
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as i32, Number1::One as i32);

    // 当使用结构体或枚举的结构体变体时，需要显式指定每个字段的名称和值。这是因为 Rust 强调明确和可读的代码结构，防止在初始化时混淆字段的顺序。
    let msg1 = Message::Move{ x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write("AABB".to_string()); // 使用 "hello, world!" 来初始化

    let msgs:[Message;3] = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(&msg)
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
} 

fn show_message(msg: &Message) {
    println!("{:?}", msg);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}