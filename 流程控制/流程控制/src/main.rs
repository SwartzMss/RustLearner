
// 填空
fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");

            10 * n
        } else {
            println!("数字太大，我们得让它减半");

            n / 2.0 as i32
        };

    println!("{} -> {}", n, big_n);


    let a = [4,3,2,1];

    // 通过索引和值的方式迭代数组 `a` 
    for (i,v) in a.iter().enumerate() {
        println!("第{}个元素是{}",i+1,v);
    }
} 