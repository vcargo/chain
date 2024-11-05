use crate::bindings::event_emitter::UintKeyValue;
use ethers::prelude::U256;
use strum_macros::{Display, EnumIter};

pub mod bindings;

// https://github.com/gmx-io/gmx-synthetics/blob/main/contracts/order/Order.sol#L12
#[derive(Debug, PartialEq, Display, EnumIter)]
pub enum OrderType {
    MarketSwap,
    LimitSwap,
    MarketIncrease,
    LimitIncrease,
    MarketDecrease,
    LimitDecrease,
    StopLossDecrease,
    Liquidation,
    StopIncrease,
}

impl TryFrom<U256> for OrderType {
    type Error = String;

    fn try_from(value: U256) -> Result<Self, Self::Error> {
        match value.as_u64() {
            0 => Ok(OrderType::MarketSwap),
            1 => Ok(OrderType::LimitSwap),
            2 => Ok(OrderType::MarketIncrease),
            3 => Ok(OrderType::LimitIncrease),
            4 => Ok(OrderType::MarketDecrease),
            5 => Ok(OrderType::LimitDecrease),
            6 => Ok(OrderType::StopLossDecrease),
            7 => Ok(OrderType::Liquidation),
            8 => Ok(OrderType::StopIncrease),
            _ => Err(format!("Invalid order type value: {}", value)),
        }
    }
}

pub fn find_uint_items_value(items: &[UintKeyValue], key: &str) -> anyhow::Result<U256> {
    items
        .iter()
        .find(|item| item.key == key)
        .map(|item| item.value)
        .ok_or(anyhow::anyhow!("'{key}' key not found"))
}

pub fn get_order_type(items: &[UintKeyValue]) -> anyhow::Result<OrderType> {
    let order_type = find_uint_items_value(&items, "orderType")?;
    OrderType::try_from(order_type).map_err(|e| anyhow::anyhow!(e))
}
