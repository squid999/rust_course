# Lecture 01

# Description

创建一个Rust工程，
* 添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
* 添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符 ● 使用Cargo编译运行此工程
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
ch: A
ch: B
ch: C
ch: D
ch: E
ch: F
ch: G
ch: H
ch: I
ch: J
ch: K
ch: L
ch: M
ch: N
ch: O
ch: P
ch: Q
ch: R
ch: S
ch: T
ch: U
ch: V
ch: W
ch: X
ch: Y
ch: Z
ch: [
ch: \
ch: ]
ch: ^
ch: _
ch: `
ch: a
ch: b
ch: c
ch: d
ch: e
ch: f
ch: g
ch: h
ch: i
ch: j
ch: k
ch: l
ch: m
ch: n
ch: o
ch: p
ch: q
ch: r
ch: s
ch: t
ch: u
ch: v
ch: w
ch: x
ch: y
ch: z
```

