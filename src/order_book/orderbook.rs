use std::{any, collections::BTreeMap};
use crate::order_book::types::{CancelOrder, NewOrder, Order, OrderType, PriceLevel};
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

                let order = Order{
                    order_id : Uuid::new_v4(),
                    order_type : new_order.order_type,
                    initial_quantity : new_order.quantity,
                    current_quantity : new_order.quantity,
                    market_limit : None
                };

                if new_order.is_buy_side {
                    if let Err(error) = self.create_buy_order(order, new_order.price.unwrap()){
                        //log the error
                    };
                    Ok(())
                }
                else {
                    if let Err(error) = self.create_sell_order(order, new_order.price.unwrap()){
                        // log the error
                    };
                    Ok(())
                }
    }

    pub fn create_buy_order(&mut self, order : Order, price : u32) -> Result<(), anyhow::Error>{
        
            // before creating pricelevel check for exsiting value
            if let Some(existing_index) = self._bid._price_map.get(&price){
                if let Some(unconfirmed_price_level) = self._bid._price_level.get_mut(*existing_index){ // get_mut returns you Option<&mut V>
                    if let Some(existing_price_level) = unconfirmed_price_level{
                        existing_price_level.insert(order);
                        return Ok(()); // we must exit as soon as the order is inserted into existing price_level
                    }
                };
            }
            let mut price_level = PriceLevel::new();
            price_level.insert(order);
            if let Some(free_index) = self._bid._free_list.pop(){
                self._bid._price_level.insert(free_index, Some(price_level));
                self._bid._price_map.entry(price).or_insert(free_index);
                return Ok(());
            }
            self._bid._price_level.push(Some(price_level));
            let index = self._bid._price_level.len() - 1;
            self._bid._price_map.entry(price).or_insert(index);
        
        Ok(())
    }
    pub fn create_sell_order(&mut self, order : Order, price : u32) -> Result<(), anyhow::Error>{
        // before creating pricelevel check for exsiting value
            if let Some(existing_index) = self._ask._price_map.get(&price){
                if let Some(unconfirmed_price_level) = self._ask._price_level.get_mut(*existing_index){ // get_mut returns you Option<&mut V>
                    if let Some(existing_price_level) = unconfirmed_price_level{
                        existing_price_level.insert(order);
                        return Ok(()); // we must exit as soon as the order is inserted into existing price_level
                    }
                };
            }
            let mut price_level = PriceLevel::new();
            price_level.insert(order);
            if let Some(free_index) = self._ask._free_list.pop(){
                self._ask._price_level.insert(free_index, Some(price_level));
                self._ask._price_map.entry(price).or_insert(free_index);
                return Ok(());
            }
            self._ask._price_level.push(Some(price_level));
            let index = self._ask._price_level.len() - 1;
            self._ask._price_map.entry(price).or_insert(index);
            Ok(())
    }
    pub fn cancel_order(&mut self, order : CancelOrder) -> Result<(), anyhow::Error>{
        if order.is_buy_side {
            if let Some(existing_index) = self._bid._price_map.get(&order.price){
                if let Some(unconfirmed_price_level) = self._bid._price_level.get_mut(*existing_index){
                    if let Some(existing_price_level) = unconfirmed_price_level{
                        if let Err(error) = existing_price_level.remove(order){
                            // add deletion error over here
                        };
                        return Ok(())
                    }
                }
            }
        } else {
            if let Some(existing_index) = self._ask._price_map.get(&order.price){
                if let Some(unconfirmed_price_level) = self._ask._price_level.get_mut(*existing_index){
                    if let Some(existing_price_level) = unconfirmed_price_level{
                        if let Err(error) = existing_price_level.remove(order){
                            // add deletion error over here
                        };
                        return Ok(())
                    }
                }
            }
        }
        Ok(())
    }
    pub fn modify_order(&mut self){
        
    }
}

#[derive(Debug)]
pub struct HalfBook{
    _price_map : BTreeMap<u32, usize>,
    _price_level : Vec<Option<PriceLevel>>,
    _free_list : Vec<usize> // we're storing the free indices from the price level to keep the cache lines hot.
}

impl HalfBook {
    pub fn new() -> Self{
        Self { _price_map: BTreeMap::new(), _price_level: Vec::new(), _free_list: Vec::new() }
    }
}