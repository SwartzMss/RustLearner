fn main() {
    println!("Hello, world!");
    let mut s = String::from("value"); // 从字面值构造string类型 是再堆上的
    println!("s = {}",s);
    s.push_str(" good");
    println!("s = {}" ,s);


    let x1 = 5;
    let x2 = x1;
    println!("x1 = {}, x2= {}", x1,x2);

    let s1 = String::from("aaa");
    //let s2 = s1; // 堆上的内容和上面的x1 和x2上的内存是不一样的
    // println!("s1 = {}, s2= {}", s1,s2); // ^^ value borrowed here after move
    let s2 = s1.clone(); 
    println!("s1 = {}, s2= {}", s1,s2); // ^^ value borrowed here after move

    let  aa1 = String::from("aaaa");

    let aa2 = &aa1;
    let aa3 = &aa1; // 不可以同时用用可变和不可变引用
    // let aa4 = &mut aa1; //cannot borrow `aa1` as mutable because it is also borrowed as immutable
    println!("aa2 = {}, aa3= {}", aa2,aa3);


    // 不需要额外的使用struct
    #[derive(Debug)]
    enum IpAddrForMe
    {
        V4(String),
        V6(String),
    }

    let v4_ip1: IpAddrForMe = IpAddrForMe::V4(String::from("127.0.0.1"));
    let v6_ip1: IpAddrForMe = IpAddrForMe::V6(String::from("127.0.0.1"));
    println!("home = {:#?}, v6IP1= {:#?}", v4_ip1,v6_ip1);


    let _some_num = Some(5);
    let _some_string = Some("abcdefg");
    let _absent_num:Option<i32> = None;

    let ac:i8 = 5;
    let ac1:Option<i8> = Some(10);

    // let sum_value = ac + ac1;
    let sum_value = ac + ac1.unwrap_or_default();
}


