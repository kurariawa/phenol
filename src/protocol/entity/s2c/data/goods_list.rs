use serde::Deserialize;

#[derive(Deserialize)]
pub struct S2CDataGoodsList {
    pub success: bool,
    pub list: Vec<Goods>,
    #[serde(rename = "hasNext")]
    pub has_next: bool,
    #[serde(rename = "nextScore")]
    pub next_score: i64,
}

#[derive(Deserialize)]
pub struct Goods {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "avgPrice")]
    pub avg_price: i32,
    #[serde(rename = "buyType")]
    pub buy_type: i32,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "goodsId")]
    pub goods_id: String,
    #[serde(rename = "goodsName")]
    pub goods_name: String,
    pub imgs: Vec<String>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    #[serde(rename = "minPrice")]
    pub min_price: i32,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "goodsId_")]
    pub goods_id_: String,
    pub deleted: bool,
    pub status: i32,
}
