fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // ok: 符合第一种
    foo::<2021>(); // ok: 符合第二种
    foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: 符合第三种
    
    // foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
    // foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T
    
    let _: [u8; M]; // ok: 符合第一种
    // let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
}



// 修复错误
struct Array<T, const N: usize> {
    data : [T; N]
}


// 填空
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [4, 5, 6],
        },
        Array {
            data: [1, 2,5]
        }
    ];

    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
