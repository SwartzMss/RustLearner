
fn main() 
{
    let x = 111;
    match_number(x);
}
fn match_number(n: i32) {
    match n {
        // 匹配一个单独的值
        1 => println!("One!"),
        // 使用 `|` 填空，不要使用 `..` 或 `..=`
        2|3|4|5 => println!("match 2 -> 5"),
        // 匹配一个闭区间的数值序列
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }

    // 填空，让 p 匹配第二个分支
    let p = Point { x: 4, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 第二个分支
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:id@ 3..=7,
        } => println!("id 值的范围在 [3, 7] 之间: {}", id),
        Message::Hello { id: newid@(10 | 11 | 12) } => {
            println!("id 值的范围在 [10, 12] 之间: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, _, _, _, _, _, _, _, _, _, last)=> {  // 只有元组才可以这样去匹配 （——，——）
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }

    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value => value.push_str(" world!") 
    }
}


struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}