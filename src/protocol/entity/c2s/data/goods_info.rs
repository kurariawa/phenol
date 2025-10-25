use serde::Serialize;

#[derive(Serialize)]
pub struct C2SDataGoodsInfo {
    #[serde(rename = "goodsId")]
    goods_id: String,
    #[serde(rename = "groupId")]
    group_id: String,
    #[serde(rename = "userId")]
    user_id: String,
}

impl C2SDataGoodsInfo {
    pub fn new(goods_id: String, group_id: String, user_id: String) -> Self {
        Self {
            goods_id,
            group_id,
            user_id,
        }
    }
}
