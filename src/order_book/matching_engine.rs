use crate::order_book::{orderbook::OrderBook, types::{NewOrder, OrderType}};

#[derive(Debug)]
pub struct MatchingEngine{
    _orderbook : OrderBook
}

impl MatchingEngine {
    pub fn handle_new_order(&mut self, order : NewOrder){
        if (order.is_buy_side == true){
            self.match_order(order.order_type);
        }
    }

    pub fn match_order(&mut self, order_type : OrderType){
        match order_type{
            OrderType::Market(None) => {
                
            },
            OrderType::Market(Some(_)) => {

            }
            OrderType::Limit => {

            }
        }
    }


}

