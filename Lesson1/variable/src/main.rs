fn main() {
    let x = 5;
    println!("x = {}", x);

    // x = 6; // cannot assign twice to immutable variable
    let x = x + 1; //shadow  他和mut是不一样的
    println!("x = {}", x);

    let space = "sasas";
    println!("space = {}", space);
    let space = space.len();// shadow的话 类型也是可以改变的
    println!("space = {}", space);

    const MAX_POINTS:i128 = 1277;
    println!("MAX_POINTS = {}", MAX_POINTS);

    let tup:(i32, f64, u8) = (500,6.4,4);
    let (x,y,z) = tup;
    println!("{} {} {}",x,y,z);
    println!("{} {} {}",tup.0,tup.1,tup.2);

    let a = [1,2,3,4,5];
    let b = [1;3];
    println!("{} {} {}",a[0],a[1],a[2]);
    println!("{} {} {}",b[0],b[1],b[2]);

    let ax = 5;

    let ay = {
        let ax = 1;
        ax + 1
    };

    println!("ay = {}, ax = {}", ay, ax);
    println!("funcx = {}", funcx());

    let num = 5;
    if num<10{
        println!(" < 10");
    }else {
        println!(">10")
    }
}


fn funcx() ->i32
{
    10
}