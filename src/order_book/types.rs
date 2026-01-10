use std::collections::{HashMap, VecDeque};

use uuid::Uuid;

#[derive(Debug)]
pub struct Order{
    pub order_id : Uuid,
    pub order_type : OrderType,
    pub initial_quantity : u32,
    pub current_quantity : u32,
    pub market_limit : Option<f64>
}


#[derive(Debug)]
pub struct NewOrder{
    pub uuid : String, //remains the same for every order
    pub order_id : Uuid, // changes, could be multiple order for different assets/security
    pub price : Option<u32>, // would be None if its a market order
    pub quantity : u32,
    pub is_buy_side : bool,
    pub security_id : u32,
    pub order_type : OrderType
}

#[derive(Debug)]
pub enum OrderType{
    Market(Option<f64>), // No cieling/floor price. leftover quantity is canceled
    Limit
}

#[derive(Debug)]
pub struct CancelOrder{
    pub order_id : Uuid,
    pub price : u32,
    pub is_buy_side : bool
}

pub struct ModifyOrder{
    pub order_id : Uuid,
    pub is_buy_side : bool,
    pub modify_price : u32,
    pub modify_quantity : u32
}

pub struct AssetLookUp{
    _asset_view : HashMap<String, String>
}

impl AssetLookUp {
    pub fn new() -> Self{
        Self { _asset_view: HashMap::new() }
    }
    pub fn insert(&mut self, asset_id : String) -> Result<(), anyhow::Error >{
        if let Err(e) = self.insert(asset_id){

        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct PriceLevel{
    _price_level : VecDeque<Order>
}

impl PriceLevel {
    pub fn new() -> Self{
        Self { _price_level: VecDeque::new() }
    }
    pub fn insert(&mut self, order : Order){
        self._price_level.push_back(order);
        // add insertion logging here
    }
    pub fn remove(&mut self, order : CancelOrder) -> Result<() , anyhow::Error>{
        self._price_level.retain(|existing_order| {
         existing_order.order_id != order.order_id    
        });
        // add deletion logging here
        Ok(())
    }
    pub fn order_range(&mut self){

    }
}


