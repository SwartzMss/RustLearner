use std::io;
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("猜数");
    println!("猜测一个数字");
    let _aa = 2;
    // aa = 123; 默认不可修改
    let secret_num = rand::thread_rng().gen_range(1, 100);
    // println!("secret_num = {}",secret_num);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取");
    
        let guess:i128 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("猜测的数组是：{}" , guess);
    
        match guess.cmp(&secret_num)
        {
            Ordering::Less =>println!("too small!"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal =>{println!("equal!");break;}
        }
    }


}
