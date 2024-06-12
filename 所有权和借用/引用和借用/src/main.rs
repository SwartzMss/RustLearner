
fn main() {
    // let x = 5;
    // // 填写空白处
    // let p = &x;
    // println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84

    // let x = 5;
    // let y = &x;
    // // 只能修改以下行
    // assert_eq!(5, *y);

    // let mut s = String::from("hello, ");

    // borrow_object(&s)

    // let mut s = String::from("hello, ");
    // push_str(&mut s)

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // println!("{}", r1); // 使用 r1 后
    // let r2 = &mut s; // 再创建 r2
    // println!("{}", r2);


 }

fn push_str(s: &mut String) {
    s.push_str("world")
}
 

fn borrow_object(s: &String) {}