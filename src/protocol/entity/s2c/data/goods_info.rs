use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct S2CDataGoodsInfo {
    pub success: bool,
    pub goods: Goods,
    #[serde(rename = "userRole")]
    pub user_role: i32,
    pub group: Group,
}

#[derive(Deserialize, Debug)]
pub struct Goods {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "avgPrice")]
    pub avg_price: i32,
    #[serde(rename = "boxPrice")]
    pub box_price: i32,
    #[serde(rename = "buyType")]
    pub buy_type: i32,
    pub cancelable: bool,
    #[serde(rename = "createOpenId")]
    pub create_open_id: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "createUserId")]
    pub create_user_id: String,
    #[serde(rename = "createUserId_")]
    pub create_user_id_: String,
    pub deleted: bool,
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "goodsId")]
    pub goods_id: String,
    #[serde(rename = "goodsName")]
    pub goods_name: String,
    #[serde(rename = "groupBoxNum")]
    pub group_box_num: i32,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "groupId_")]
    pub group_id_: String,
    pub imgs: Vec<String>,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    #[serde(rename = "itemIds")]
    pub item_ids: Vec<String>,
    #[serde(rename = "minPrice")]
    pub min_price: i32,
    #[serde(rename = "modifyTime")]
    pub modify_time: i64,
    #[serde(rename = "modifyUserId_")]
    pub modify_user_id_: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "goodsId_")]
    pub goods_id_: String,
    #[serde(rename = "createUser")]
    pub create_user: CreateUser,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userId_")]
    pub user_id_: String,
    #[serde(rename = "coserName")]
    pub coser_name: String,
    pub avatar: String,
    #[serde(rename = "qqName")]
    pub qq_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "itemName")]
    pub item_name: String,
    #[serde(rename = "itemImg")]
    pub item_img: String,
    #[serde(rename = "itemAllNum")]
    pub item_all_num: i32,
    #[serde(rename = "itemOrderedNum")]
    pub item_ordered_num: i32,
    #[serde(rename = "itemPrice")]
    pub item_price: i32,
}

#[derive(Deserialize, Debug)]
pub struct Group {
    #[serde(rename = "groupAvatar")]
    pub group_avatar: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
}
