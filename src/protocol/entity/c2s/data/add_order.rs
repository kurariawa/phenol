use serde::Serialize;

#[derive(Serialize)]
pub struct OrderNode {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "orderNum")]
    pub order_num: i32,
}

#[derive(Serialize)]
pub struct C2SDataAddOrder {
    #[serde(rename = "createUserId_")]
    create_user_id: String,
    #[serde(rename = "groupId")]
    group_id: String,
    #[serde(rename = "goodsId")]
    goods_id: String,
    #[serde(rename = "ownerUserId")]
    owner_user_id: String,
    #[serde(rename = "ownerUserId_")]
    owner_user_id_plus: String,
    source: &'static str,
    #[serde(rename = "orderList")]
    order_list: Vec<OrderNode>,
}

impl C2SDataAddOrder {
    pub fn new(
        create_user_id: String,
        group_id: String,
        goods_id: String,
        owner_user_id: String,
        owner_user_id_plus: String,
        order_list: Vec<OrderNode>,
    ) -> Self {
        Self {
            create_user_id,
            group_id,
            goods_id,
            owner_user_id,
            owner_user_id_plus,
            source: "own",
            order_list,
        }
    }
}
