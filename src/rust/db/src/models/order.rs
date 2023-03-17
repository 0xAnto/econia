use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

use crate::schema::orders;

#[derive(Debug, DbEnum, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::Side"]
pub enum Side {
    Buy,
    Sell,
}

impl From<types::Side> for Side {
    fn from(value: types::Side) -> Self {
        match value {
            types::Side::Buy => Self::Buy,
            types::Side::Sell => Self::Sell,
        }
    }
}

impl From<Side> for types::Side {
    fn from(value: Side) -> Self {
        match value {
            Side::Buy => Self::Buy,
            Side::Sell => Self::Sell,
        }
    }
}

#[derive(Debug, DbEnum, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::OrderState"]
pub enum OrderState {
    Open,
    Filled,
    Cancelled,
    Evicted,
}

#[derive(Clone, Debug, Queryable)]
pub struct Order {
    pub market_order_id: BigDecimal,
    pub market_id: BigDecimal,
    pub side: Side,
    pub size: BigDecimal,
    pub price: BigDecimal,
    pub user_address: String,
    pub custodian_id: Option<BigDecimal>,
    pub order_state: OrderState,
    pub remaining_size: BigDecimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = orders)]
pub struct NewOrder<'a> {
    pub market_order_id: &'a BigDecimal,
    pub market_id: &'a BigDecimal,
    pub side: &'a Side,
    pub size: &'a BigDecimal,
    pub price: &'a BigDecimal,
    pub user_address: &'a str,
    pub custodian_id: Option<BigDecimal>,
    pub order_state: &'a OrderState,
    pub remaining_size: &'a BigDecimal,
    pub created_at: &'a DateTime<Utc>,
}
