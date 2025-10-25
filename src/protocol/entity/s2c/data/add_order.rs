use serde::Deserialize;

#[derive(Deserialize)]
pub struct S2CDataAddOrder {
    pub success: bool,
    pub order: Order,
}

#[derive(Deserialize)]
pub struct Order {
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub items: Vec<OrderItem>,
    #[serde(rename = "orderTime")]
    pub order_time: i64,
}

#[derive(Deserialize)]
pub struct OrderItem {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "orderNum")]
    pub order_num: i32,
    #[serde(rename = "itemPrice")]
    pub item_price: i32,
    #[serde(rename = "itemName")]
    pub item_name: String,
    #[serde(rename = "itemImg")]
    pub item_img: String,
    #[serde(rename = "itemOrderIds")]
    pub item_order_ids: Vec<String>,
}
