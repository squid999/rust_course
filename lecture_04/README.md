Lecture 04
===============

## Project 
### Proj 1.
#### 定义三种不同类型的交易订单
```rust
#[derive(Debug)]
pub struct MarketOrder {
    order_id: String, // 订单号
    contract: String, // 合约
    multiplier: u32, // 合约乘数
    volume: u32, // 手数
}

#[derive(Debug)]
pub struct LimitOrder {
    order_id: String,
    contract: String,
    multiplier: u32,
    price: f64,
    volume: u32,
}

#[derive(Debug)]
pub struct StopOrder {
    order_id: String,
    contract: String,
    price: f64,
    volume: u32,
}
```

#### 定义各类订单共同接口
```rust
impl MarketOrder {
    pub fn new(order_id: String, contract: String, volume: u32, multiplier: u32) -> MarketOrder {
        MarketOrder {
            order_id,
            contract,
            volume,
            multiplier,
        }
    }
    pub fn detail(&self) -> String {
        format!(
            "MarketOrder[{}] <{}>:  {}@?",
            self.order_id, self.contract, self.volume
        )
    }
}

impl LimitOrder {
    pub fn new(
        order_id: String,
        contract: String,
        price: f64,
        volume: u32,
        multiplier: u32,
    ) -> LimitOrder {
        LimitOrder {
            order_id,
            contract,
            price,
            volume,
            multiplier,
        }
    }
    pub fn detail(&self) -> String {
        format!(
            "LimitOrder[{}] <{}>: {}@{}",
            self.order_id, self.contract, self.volume, self.price
        )
    }
}

impl StopOrder {
    pub fn new(order_id: String, contract: String, price: f64, volume: u32) -> StopOrder {
        StopOrder {
            order_id,
            contract,
            price,
            volume,
        }
    }
    pub fn detail(&self) -> String {
        format!(
            "StopOrder[{}] <{}>: {}@{}",
            self.order_id, self.contract, self.volume, self.price
        )
    }
}

```

#### Case 1:
使用枚举包裹三个不同的类型，并放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法

```rust

use super::order_trait_object as oto;

pub fn poll_orders() {
    let order1 = OrderType::MarketOrder(oto::MarketOrder::new(
        "mkt".to_string(),
        "BTC/UTC".to_string(),
        1,
        10,
    ));
    let order2 = OrderType::LimitOrder(oto::LimitOrder::new(
        "lmt".to_string(),
        "BTC/UTC".to_string(),
        40000f64,
        1,
        10,
    ));
    let order3 = OrderType::StopOrder(oto::StopOrder::new(
        "stp".to_string(),
        "BTC/UTC".to_string(),
        20000f64,
        1,
    ));

    let incoming_orders = vec![order1, order2, order3];

    for order in incoming_orders.iter() {
        match order {
            OrderType::MarketOrder(mkt) => {
                println!("{}", mkt.detail());
            }
            OrderType::LimitOrder(lmt) => {
                println!("{}", lmt.detail());
            }
            OrderType::StopOrder(stp) => {
                println!("{}", stp.detail());
            }
        }
    }
}

```
运行结果
```shell
MarketOrder[mkt] <BTC/UTC>:  1@?
LimitOrder[lmt] <BTC/UTC>: 1@40000
StopOrder[stp] <BTC/UTC>: 1@20000
```

#### Case 2:
定义三个不同的类型，使用 Trait Object，将其放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法。同时，说明其上两种不同实现方法的区别。

##### 定义 Order trait 
```rust
trait Order {
    fn turnover(&self) -> f64; // 获得成交量
    fn order_id(&self) -> String; // 获得订单号
}
```
    
##### 模拟的一个轮询订单并获得成交量的流程
```rust
pub fn poll_orders() {
    let mut incoming_orders: Vec<Box<dyn Order>> = Vec::new();

    for i in 0..16 {
        let order_id = format!("order_{:05}", i);
        match i % 3 {
            0 => {
   
                let order = Box::new(MarketOrder {
                    order_id: order_id,
                    price: 100f64,
                    volume: i + 1,
                    multiplier: 10,
                });
                incoming_orders.push(order);
            }
            1 => {
      
                let order = Box::new(LimitOrder {
                    order_id: order_id,
                    price: 100f64,
                    volume: i,
                    multiplier: 10,
                });
                incoming_orders.push(order);
            }
            2 => {
 
                let order = Box::new(StopOrder {
                    order_id: order_id,
                    price: 100f64,
                    volume: i,
                });
                incoming_orders.push(order);
            }
            _ => {
                println!("execuse me ?");
            }
        }
    }

    for (i, order) in incoming_orders.iter().enumerate() {
        println!(
            "Order {}: {} turnover is {}",
            i,
            order.order_id(),
            order.turnover()
        );
    }
}
```

### Proj 2.
搜索相关文档，为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用 Trait Object 实现类型方法的调用

#### 自定义矩阵类型 

```rust
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Default + Debug,
    {
        Matrix::from_iter(rows, cols, (0..).map(|_| T::default()))
    }

    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
```
#### 实现加法运算

```rust
impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

```
#### 使用 Trait Object 实现类型方法的调用 
定义一个 **Printable trait**, 定义打印输出接口
```rust
pub trait Printable {
    fn print(&self);
}

impl<T> Printable for Matrix<T>
where
    T: Default + Debug,
{
    fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{:4?} ", self.data[i * self.rows + j]) // 此处需要给 T 类型加上 Debug 约束，否则编译不过
            }
            println!()
        }
    }
}

// 实现一个调用函数

```

#### 运行输出
```rust
fn main() {
    //...
    let m1 = Matrix:<u32>::from_iter(3, 3, (0..));
    matrix::print_pretty(&m1);
}
```
```shell
   0    1    2 
   3    4    5 
   6    7    8 
```
