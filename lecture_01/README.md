# Lecture 01

# Description

创建一个Rust工程，
* 添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
* 添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
* 使用Cargo编译运行此工程

# 目录结构
```
.
├── Cargo.toml
├── README.md
└── src
    ├── loop_01.rs
    ├── loop_02
    │   ├── mod.rs
    │   └── print_fn.rs
    └── main.rs
```

# 运行

```shell
cargo run --release
Compiling lecture_01 v0.1.0 (/Users/neuron/prj/rust/rust_course/lecture_01)
    Finished release [optimized] target(s) in 0.41s
     Running `/Users/neuron/prj/rust/rust_course/target/release/lecture_01`
submod loop1 print ['a'..'Z']
ch: a
ch: `
ch: _
ch: ^
ch: ]
ch: \
ch: [
ch: Z

submod loop2 print ['A'..'z']
A B C D E F G 
H I J K L M N 
O P Q R S T U 
V W X Y Z [ \ 
] ^ _ ` a b c 
d e f g h i j 
k l m n o p q 
r s t u v w x 
y z 
```