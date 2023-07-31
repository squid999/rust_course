Lecture 05
===============

实现：一个简单的声明宏并理解其代码结构，和编译过程。


### 示例

Rust 自带的宏 **vec![]** 是一个常用的声明宏 ，我们通常的使用方式是
```rust
fn main() {
    let a:Vec<i32> = vec![]; // 空数组
    let b = vec![1; 10]; // [1, 1, 1, 1, 1, 1, 1, 1, 1, 1] 总共10个元素
    let c = vec![0, 1, 2, 3, 4, 5]; // [0, 1, 2, 3, 4, 5]
}
```

官方的实现参考以下代码块
```rust
macro_rules! __rust_force_expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! vec {
    () => (
        $crate::__rust_force_expr!($crate::vec::Vec::new())
    );
    ($elem:expr; $n:expr) => (
        $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        $crate::__rust_force_expr!(<[_]>::into_vec(box [$($x),+]))
    );
}
```
可以看到该宏有三个匹配模式
1. 没有任何参数，返回一个空的数组
2. 有两个参数，声明方式参考数组，采用 `;` 作为两个参数的分隔符
3. 一个或者多个参数，用 `,` 作为分隔符，声明的多个参数作为数组初始化的元素

参考 `vec!` 实现一个 `hashmap!` 使用方式如下

```rust 
fn main() {
    let m = hashmap!{
        "a" => 1,
        "b" => 2,
    };
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("b"), Some(&2));
    assert_eq!(m.len(), 2);
}
```
1. 参数不固定
2. 参数形式为 $\$key$ => $\$value$

仿照 `vec!` 有以下实现
```rust
macro_rules! hashmap_v1 {
    () => { 
        {::std::collections::HashMap::new()}
    };
    // 匹配多个 $key => $value，分隔符为(,)
    // 最后 $(,)* 表示最后的(,) 可以匹配多个或没有，如果改为$(,)？则只能匹配一个或者 0 个，与正则表达一致
    ($($key: expr => $value: expr),* $(,)*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}
```

用 `cargo expand` 展开一下 `main.rs` 中的 sample1 代码段:
```rust
fn sample1() {
    let map = hashmap_v1!("a"=>1, "b"=> 2, "c" =>3, ,,,);
    for (key, value) in &map {
        println!("{} => {}", key, value)
    }
    assert_eq!(map.get("c"), Some(&3))
}
```

展开代码后
```rust
fn sample1() {
    let map = {
        let mut _map = ::std::collections::HashMap::new();
        _map.insert("a", 1);
        _map.insert("b", 2);
        _map.insert("c", 3);
        _map
    };
    for (key, value) in &map {
        {
            ::std::io::_print(format_args!("{0} => {1}\n", key, value));
        }
    }
    match (&map.get("c"), &Some(&3)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    }
}
```

还可以尝试在 `value` 中再放入 `Hashmap`

```rust
 let m2 = hashmap_v1!(
        "a"  => hashmap_v1!(),
        "b"  => hashmap_v1!(
            22 => 220,
            33 => 330,
        )
    );

    for (key, value) in &m2 {
        println!("{} => {:?}", key, value);
    }
```
cargo expand 展开后

```rust
 let m2 = {
        let mut _map = ::std::collections::HashMap::new();
        _map.insert("a", { ::std::collections::HashMap::new() });
        _map.insert(
            "b",
            {
                let mut _map = ::std::collections::HashMap::new();
                _map.insert(22, 220);
                _map.insert(33, 330);
                _map
            },
        );
        _map
    };
    for (key, value) in &m2 {
        {
            ::std::io::_print(format_args!("{0} => {1:?}\n", key, value));
        };
    }
```