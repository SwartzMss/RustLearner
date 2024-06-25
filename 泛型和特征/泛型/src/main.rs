
// 填空
struct A;          // 具体的类型 `A`.
struct S(A);       // 具体的类型 `S`.
struct SGen<T>(T); // 泛型 `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn sum<T: std::ops::Add<Output = T>>(a:T,b:T) ->T
{
    a + b
}

struct Point<T,U> 
{
    x: T,
    y: U,
}

impl<T,U>Point<T,U>
{
    fn mixup<V,W>(self, other:Point<V,W>) -> Point<T,W>
    {
        Point{x:self.x, y:other.y}
    }
}

struct PointX<T> {
    x: T,
    y: T,
}

impl PointX<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = PointX { x: 5.0, y: 10.0 }; // 类型会自动推断为 Point<f64>
    println!("{}", p.distance_from_origin());
    // 使用非泛型函数
    reg_fn(S(A));          // 具体的类型
    gen_spec_t(SGen(A));   // 隐式地指定类型参数 `A`.
    gen_spec_i32(SGen(32)); // 隐式地指定类型参数`i32`.

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式地指定类型参数 `char`.
    generic(SGen('b'));


    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point{x: 5, y : "hello".to_string()};

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}