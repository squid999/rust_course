#![allow(unused)]

use std::collections::HashMap;

macro_rules! hashmap_v1 {
    () => {
        {::std::collections::HashMap::new()}
    };
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

macro_rules! hashmap_v2 {
   ($($key:expr => $value:expr,)+) => { hashmap_v2!($($key => $value),+) };
   ($($key:expr => $value:expr),*) => {
       {
           let mut _map = ::std::collections::HashMap::new();
           $(
               _map.insert($key, $value);
           )*
           _map
       }
   };
}

fn sample1() {
    let map = hashmap_v1!("a"=>1, "b"=> 2, "c" =>3);
    for (key, value) in &map {
        println!("{} => {}", key, value)
    }
    assert_eq!(map.get("c"), Some(&3))
}

fn sample2() {
    use std::collections::HashMap;
    let m1: HashMap<&str, u32> = hashmap_v2!();
    let expected1: HashMap<&str, u32> = HashMap::new();

    assert_eq!(m1, expected1);

    let m2: HashMap<&str, u32> = hashmap_v2!("a" => 1, "b" => 2);
    let mut expected2: HashMap<&str, u32> = HashMap::new();
    expected2.insert("a", 1);
    expected2.insert("b", 2);

    assert_eq!(m2, expected2);

    let mut expected = HashMap::new();
    expected.insert("non-empty", {
        let mut subhashmap = HashMap::new();
        subhashmap.insert(23, 623);
        subhashmap.insert(34, 21);
        subhashmap
    });
    expected.insert("empty", HashMap::new());
    assert_eq!(
        hashmap_v2!(
            "non-empty" => hashmap_v2!(
                23 => 623,
                34 => 21
            ),
            "empty" => hashmap_v2!()
        ),
        expected
    );
}

fn main() {
    sample1();
    sample2();
}
