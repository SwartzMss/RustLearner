fn main() {
    println!("Hello, world!");
}


// 创建项目目录和初始化 Cargo 项目：
// 首先创建项目目录，然后初始化一个新的 Cargo 项目：

// sh
// 复制代码
// mkdir a-package
// cd a-package
// cargo init --lib
// 创建二进制包目录：
// 创建 src/bin 目录，用于存放多个二进制包：

// sh
// 复制代码
// mkdir -p src/bin
// 创建二进制包文件：
// 在 src/bin 目录下创建三个二进制包文件：

// sh
// 复制代码
// touch src/bin/hello-package.rs
// touch src/bin/main1.rs
// touch src/bin/main2.rs
// 修改 Cargo.toml 文件：
// 编辑 Cargo.toml 文件，添加二进制包配置：

// toml
// 复制代码
// [package]
// name = "a-package"
// version = "0.1.0"
// edition = "2018"

// [dependencies]

// [[bin]]
// name = "hello-package"
// path = "src/bin/hello-package.rs"

// [[bin]]
// name = "main1"
// path = "src/bin/main1.rs"

// [[bin]]
// name = "main2"
// path = "src/bin/main2.rs"
// 创建库包文件：
// src/lib.rs 文件已经由 cargo init --lib 命令生成，你可以在其中编写库的代码。

// 编写二进制包代码：
// 在每个二进制包文件中添加一些示例代码：

// rust
// 复制代码
// // src/bin/hello-package.rs
// fn main() {
//     println!("Hello, package!");
// }

// // src/bin/main1.rs
// fn main() {
//     println!("This is main1");
// }

// // src/bin/main2.rs
// fn main() {
//     println!("This is main2");
// }
// 创建其他目录和文件：
// 根据你的需求创建测试、基准测试和示例目录：

// sh
// 复制代码
// mkdir tests
// touch tests/some_integration_tests.rs

// mkdir benches
// touch benches/simple_bench.rs

// mkdir examples
// touch examples/simple_example.rs
// 编写其他文件的代码：

// rust
// 复制代码
// // tests/some_integration_tests.rs
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_example() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// // benches/simple_bench.rs
// #[macro_use]
// extern crate criterion;
// use criterion::Criterion;

// fn simple_benchmark(c: &mut Criterion) {
//     c.bench_function("addition", |b| b.iter(|| 1 + 2));
// }

// criterion_group!(benches, simple_benchmark);
// criterion_main!(benches);

// // examples/simple_example.rs
// fn main() {
//     println!("This is an example");
// }
// 完整目录结构
// plaintext
// 复制代码
// .
// ├── Cargo.toml
// ├── Cargo.lock
// ├── src
// │   ├── lib.rs
// │   └── bin
// │       ├── hello-package.rs
// │       ├── main1.rs
// │       └── main2.rs
// ├── tests
// │   └── some_integration_tests.rs
// ├── benches
// │   └── simple_bench.rs
// └── examples
//     └── simple_example.rs
// 使用这些命令和步骤，你可以创建一个包含多个二进制包和一个库包的 Rust 项目，并按照示例代码进行实现。