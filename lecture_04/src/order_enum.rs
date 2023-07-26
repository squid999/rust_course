//定义订单提交状态

use super::order_trait_object as oto;

pub enum OrderType {
    MarketOrder(oto::MarketOrder),
    LimitOrder(oto::LimitOrder),
    StopOrder(oto::StopOrder),
}

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
