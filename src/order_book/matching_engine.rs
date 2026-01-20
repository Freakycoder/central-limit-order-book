use crate::order_book::{orderbook::OrderBook, types::{NewOrder, OrderType, PriceLevel}};

#[derive(Debug)]
pub struct MatchingEngine{
    _orderbook : OrderBook,
}

impl MatchingEngine {
    pub fn handle_new_order(&mut self, order : NewOrder) -> Result<(), anyhow::Error>{
        if order.is_buy_side {
            match order.order_type {
                OrderType::Market(None) => {
                    if order.is_buy_side {
                    // need to immediatly execute the order on the best of other half
                    let mut fill_quantity = order.quantity;
                    while fill_quantity > 0 {
                        let remove_node: bool;
                        {
                            let Some(mut price_node) = self._orderbook.bid.price_map.first_entry()
                            else {
                                break;
                            };
                            let price_level = price_node.get_mut();
                            while price_level.total_quantity > 0 && fill_quantity > 0 {
                                let head_idx = price_level.head;
                                let first_order_node = self._orderbook.bid.order_pool[head_idx].as_mut().unwrap();
                                if fill_quantity >= first_order_node.current_quantity{
                                    fill_quantity -= first_order_node.current_quantity;
                                    price_level.total_quantity -= fill_quantity;
                                    let next = first_order_node.next;
                                    self._orderbook.bid.order_pool[head_idx] = None;
                                    if let Some(next_order_idx) = next{
                                        price_level.head = next_order_idx;
                                    }
                                    else {
                                        break;
                                    }
                                } else {
                                  first_order_node.current_quantity -= fill_quantity;
                                  price_level.total_quantity -= fill_quantity;
                                  fill_quantity = 0;
                                }
                            }
                            remove_node = price_level.total_quantity == 0;
                        }
                        if remove_node{
                            self._orderbook.bid.price_map.pop_first();
                        }
                    };
                    }
                }
                OrderType::Market(market_limit) => {

                }
                OrderType::Limit => {

                }
            }
        }
        else {
            
        }
        Ok(())
    }

   


}

