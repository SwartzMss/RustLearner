
fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v = Vec::from(arr);
    is_vec(v);

    let v = vec![1, 2, 3];
    is_vec(v);

    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let v = vec!(1, 2, 3);
    is_vec(v);
    
    // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
    // 使用 Vec::new 和 `for` 来重写下面这段代码
    let mut v1 = Vec::new();
    for &item in &arr {
        v1.push(item);
    }
    is_vec(v1);


    println!("Success!");

    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend(v1.iter()); // 这是因为 v1.iter() 返回的是对 v1 元素的不可变引用 &u8。

    assert_eq!(v1, v2);

    println!("Success!");

    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.into();
 
    assert_eq!(v1, v2);
 
    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.clone().into_bytes();  // 这里需要 clone，因为下面再次使用 s

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    // 迭代器 Iterators 可以通过 collect 变成 Vec
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!")
}

fn is_vec(v: Vec<u8>) {}