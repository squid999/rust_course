use std::any::{Any, TypeId};

trait Order {
    fn turnover(&self) -> f64;
    fn order_id(&self) -> String;
}

enum OrderStatus {
    Submitted,
    Accepted,
    Rejected,
    Canceled,
}

#[derive(Debug)]
struct MarketOrder {
    order_id: String,
    multiplier: u32,
    price: f64,
    volume: u32,
}

#[derive(Debug)]
struct LimitOrder {
    order_id: String,
    multiplier: u32,
    price: f64,
    volume: u32,
}

#[derive(Debug)]
struct StopOrder {
    order_id: String,
    price: f64,
    volume: u32,
}

impl MarketOrder {
    fn new(order_id: String, price: f64, volume: u32, multiplier: u32) -> MarketOrder {
        MarketOrder {
            order_id,
            price,
            volume,
            multiplier,
        }
    }
}

impl LimitOrder {
    fn new(order_id: String, price: f64, volume: u32, multiplier: u32) -> LimitOrder {
        LimitOrder {
            order_id,
            price,
            volume,
            multiplier,
        }
    }
}

impl StopOrder {
    fn new(order_id: String, price: f64, volume: u32) -> StopOrder {
        StopOrder {
            order_id,
            price,
            volume,
        }
    }
}

impl Order for MarketOrder {
    fn turnover(&self) -> f64 {
        self.price * (self.multiplier as f64) * (self.volume as f64)
    }

    fn order_id(&self) -> String {
        self.order_id.clone()
    }
}

impl Order for LimitOrder {
    fn turnover(&self) -> f64 {
        self.price * (self.multiplier as f64) * (self.volume as f64)
    }

    fn order_id(&self) -> String {
        self.order_id.clone()
    }
}

impl Order for StopOrder {
    fn turnover(&self) -> f64 {
        self.price * (self.volume as f64)
    }

    fn order_id(&self) -> String {
        self.order_id.clone()
    }
}

pub fn get_type_name<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

pub fn poll_orders() {
    let mut incoming_orders: Vec<Box<dyn Order>> = Vec::new();

    for i in 0..16 {
        let order_id = format!("order_{:05}", i);
        match i % 3 {
            0 => {
                // let order = MarketOrder::new(order_id, 100f64, i, 10);
                let order = Box::new(MarketOrder {
                    order_id: order_id,
                    price: 100f64,
                    volume: i + 1,
                    multiplier: 10,
                });
                incoming_orders.push(order);
            }
            1 => {
                // let order_id = format!("order_{:05}", i);
                let order = Box::new(LimitOrder {
                    order_id: order_id,
                    price: 100f64,
                    volume: i,
                    multiplier: 10,
                });
                incoming_orders.push(order);
            }
            2 => {
                // let order_id = format!("order_{:05}", i);
                // let order = MarketOrder::new(order_id, 100f64, i, 10);
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
    // println!("{}", incoming_orders.len());
    for (i, order) in incoming_orders.iter().enumerate() {
        println!(
            "Order {}: {} turnover is {}",
            i,
            order.order_id(),
            order.turnover()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dyn_type_name() {
        let order1 = MarketOrder::new("aaa".to_string(), 42f64, 1, 10);
        let n = get_type_name(&order1);

        assert_eq!(n, "lecture_04::order_trait_object::MarketOrder");
    }
}
