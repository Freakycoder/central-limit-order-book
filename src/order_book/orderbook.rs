use std::{any, collections::{BTreeMap, HashMap}};
use crate::order_book::types::{CancelOrder, ModifyOrder, NewOrder, OrderNode, OrderRegistry, OrderType, PriceLevel};
use uuid::Uuid;

#[derive(Debug)]
pub struct OrderBook{
    pub asset_id : String,
    _ask : HalfBook,
    _bid : HalfBook
}
impl OrderBook {
    pub fn new (name : String,) -> Self{
        Self { asset_id : name , _ask : HalfBook::new(), _bid : HalfBook::new() }
    }
    pub fn handle_order(&mut self, new_order : NewOrder) -> Result<(), anyhow::Error>{
        
        // if self._ask._price_map.contains_key(key){
        //     self._ask._price_map.insert(key, value);
        // }
        // if we wanted to insert a key into map, we would have first need to look up if
        // key exist, if so then insert. that would be 2 look ups (traverses) in total.
        
        // let entry_handle = self._ask._price_map.entry(order.price.unwrap()); -->  entry is a handle that checks weather the node + slot already exist or the exact position where the node needs to be inserted.
        // entry_handle.or_insert(2); -->  this consumes the entry handle given by entry and finalizes the action at the already located position
        // if the key exist at then do nothing, return mutable reference to existing value. if doesn't exist then insert the value 100 and return mut ref to it.

                if new_order.is_buy_side {
                    if let Err(error) = self.create_buy_order(new_order){
                        //log the error
                    };
                    Ok(())
                }
                else {
                    if let Err(error) = self.create_sell_order(new_order){
                        // log the error
                    };
                    Ok(())
                }
    }

    pub fn create_buy_order(&mut self, new_order : NewOrder) -> Result<(), anyhow::Error>{
        
        let mut order = OrderNode { order_id: new_order.order_id, 
            initial_quantity: new_order.quantity, 
            current_quantity: new_order.quantity, 
            market_limit: new_order.price, 
            next: None, 
            prev: None 
        };

        if let Some(price_level) = self._bid._price_map.get_mut(&new_order.price){
            order.prev = Some(price_level.tail);
            if let Some(free_index) = self._bid._free_list.pop(){
                self._bid._order_pool.insert(free_index, Some(order));
                self._bid.order_registry.insert(new_order.order_id, free_index);
                price_level.tail = free_index;
                if let Some(prev_order) = self._bid._order_pool.get_mut(price_level.tail).unwrap(){
                    prev_order.next = Some(free_index);
                };
                return Ok(());
            }
            self._bid._order_pool.push(Some(order));
            let new_tail = self._bid._order_pool.len() - 1;
            self._bid.order_registry.insert(new_order.order_id, new_tail);
            price_level.tail = new_tail;
            if let Some(prev_order) = self._bid._order_pool.get_mut(price_level.tail).unwrap(){
                prev_order.next = Some(new_tail);
            };
            return Ok(());
        }

        let mut new_price_level = PriceLevel{
            head : 0,
            tail : 0,
            order_count : 0,
            total_quantity : 0
        };
        if let Some(free_index) = self._bid._free_list.pop(){
            self._bid._order_pool.insert(free_index, Some(order));
            self._bid.order_registry.insert(new_order.order_id, free_index);
            new_price_level.head = free_index;
            new_price_level.tail = free_index;
            new_price_level.order_count += 1;
            new_price_level.total_quantity += new_order.quantity;
            self._bid._price_map.entry(new_order.price).or_insert(new_price_level);
            return Ok(())
        }
        self._bid._order_pool.push(Some(order));
        let new_index = self._bid._order_pool.len()-1;
        self._bid.order_registry.insert(new_order.order_id, new_index);
        new_price_level.head = new_index;
        new_price_level.tail = new_index;
        new_price_level.order_count += 1;
        new_price_level.total_quantity += new_order.quantity;
        self._bid._price_map.entry(new_order.price).or_insert(new_price_level);
        
        Ok(())
    }
    pub fn create_sell_order(&mut self, new_order : NewOrder) -> Result<(), anyhow::Error>{
        let mut order = OrderNode { order_id: new_order.order_id, 
            initial_quantity: new_order.quantity, 
            current_quantity: new_order.quantity, 
            market_limit: new_order.price, 
            next: None, 
            prev: None 
        };

        if let Some(price_level) = self._ask._price_map.get_mut(&new_order.price){
            order.prev = Some(price_level.tail);
            if let Some(free_index) = self._ask._free_list.pop(){
                self._ask._order_pool.insert(free_index, Some(order));
                self._ask.order_registry.insert(new_order.order_id, free_index);
                price_level.tail = free_index;
                if let Some(prev_order) = self._ask._order_pool.get_mut(price_level.tail).unwrap(){
                    prev_order.next = Some(free_index);
                };
                return Ok(());
            }
            self._ask._order_pool.push(Some(order));
            let new_tail = self._ask._order_pool.len() - 1;
            self._ask.order_registry.insert(new_order.order_id, new_tail);
            price_level.tail = new_tail;
            if let Some(prev_order) = self._ask._order_pool.get_mut(price_level.tail).unwrap(){
                prev_order.next = Some(new_tail);
            };
            return Ok(());
        }

        let mut new_price_level = PriceLevel{
            head : 0,
            tail : 0,
            order_count : 0,
            total_quantity : 0
        };
        if let Some(free_index) = self._ask._free_list.pop(){
            self._ask._order_pool.insert(free_index, Some(order));
            self._ask.order_registry.insert(new_order.order_id, free_index);
            new_price_level.head = free_index;
            new_price_level.tail = free_index;
            new_price_level.order_count += 1;
            new_price_level.total_quantity += new_order.quantity;
            self._ask._price_map.entry(new_order.price).or_insert(new_price_level);
            return Ok(())
        }
        self._ask._order_pool.push(Some(order));
        let new_index = self._ask._order_pool.len()-1;
        self._ask.order_registry.insert(new_order.order_id, new_index);
        new_price_level.head = new_index;
        new_price_level.tail = new_index;
        new_price_level.order_count += 1;
        new_price_level.total_quantity += new_order.quantity;
        self._ask._price_map.entry(new_order.price).or_insert(new_price_level);
        
        Ok(())
    }
    pub fn cancel_order(&mut self, order : CancelOrder) -> Result<(), anyhow::Error>{
        if order.is_buy_side {
           if self._bid.order_registry.order_exist(order.order_id){
                if let Some(deleted_index) = self._bid.order_registry.delete_key(order.order_id){
                    let (prev, next) = {
                        let node = self._bid._order_pool[deleted_index].as_ref().unwrap();
                        (node.prev, node.next)
                    };
                    if let Some(prev_index) = prev{
                        if let Some(possible_prev_node) = self._bid._order_pool.get_mut(prev_index){
                            if let Some(prev_node) = possible_prev_node{
                                prev_node.next = next
                            }
                        }
                    }
                    if let Some(next_index) = next{
                        if let Some(possible_next_node) = self._bid._order_pool.get_mut(next_index){
                            if let Some(next_node) = possible_next_node{
                                next_node.prev = prev
                            }
                        }
                    }
                    self._bid._order_pool.insert(deleted_index, None);
                    self._bid._free_list.push(deleted_index);
                }
           }
        } else {
           if self._ask.order_registry.order_exist(order.order_id){
                if let Some(deleted_index) = self._ask.order_registry.delete_key(order.order_id){
                    let (prev, next) = {
                        let node = self._ask._order_pool[deleted_index].as_ref().unwrap();
                        (node.prev, node.next)
                    };
                    if let Some(prev_index) = prev{
                        if let Some(possible_prev_node) = self._ask._order_pool.get_mut(prev_index){
                            if let Some(prev_node) = possible_prev_node{
                                prev_node.next = next
                            }
                        }
                    }
                    if let Some(next_index) = next{
                        if let Some(possible_next_node) = self._ask._order_pool.get_mut(next_index){
                            if let Some(next_node) = possible_next_node{
                                next_node.prev = prev
                            }
                        }
                    }
                    self._ask._order_pool.insert(deleted_index, None);
                    self._ask._free_list.push(deleted_index);
                }
           }
        }
        Ok(())
    }
    pub fn modify_order(&mut self, order : ModifyOrder){
        if order.is_buy_side{
            if self._bid.order_registry.order_exist(order.order_id){
                let idx = self._bid.order_registry.get_idx(order.order_id);
                let order_node = {
                    let node = self._bid._order_pool[*idx].as_ref().unwrap();
                    node
                };
                if order.change_side{
                    if let Err(e) = self.cancel_order(CancelOrder { order_id: order.order_id, is_buy_side: order.is_buy_side }){
                        // log the fail message - "failed to cancel the modify order"
                    };
                    // succesfully cancelled the modify order
                }
                if order.new_price != order_node.market_limit
            }
        }
    }
}

#[derive(Debug)]
pub struct HalfBook{
    _price_map : BTreeMap<u32, PriceLevel>,
    _order_pool : Vec<Option<OrderNode>>,
    _free_list : Vec<usize>, // we're storing the free indices from the price level to keep the cache lines hot.
    order_registry : OrderRegistry
}

impl HalfBook {
    pub fn new() -> Self{
        Self { _price_map: BTreeMap::new(), _order_pool: Vec::new(), _free_list: Vec::new(), order_registry : OrderRegistry::new() }
    }
}