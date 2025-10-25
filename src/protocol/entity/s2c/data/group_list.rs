use serde::Deserialize;

#[derive(Deserialize)]
pub struct S2CDataGroupList {
    pub success: bool,
    pub list: Vec<Group>,
    #[serde(rename = "hasNext")]
    pub has_next: bool,
    pub next: String,
}

#[derive(Deserialize)]
pub struct Group {
    #[serde(rename = "userRole")]
    pub user_role: i32,
    #[serde(rename = "userSort")]
    pub user_sort: i32,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "closeTime")]
    pub close_time: i64,
    #[serde(rename = "closeUserId")]
    pub close_user_id: String,
    #[serde(rename = "closeUserId_")]
    pub close_user_id_: String,
    pub closed: bool,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "createUserId")]
    pub create_user_id: String,
    #[serde(rename = "createUserId_")]
    pub create_user_id_: String,
    #[serde(rename = "goodsNum")]
    pub goods_num: i32,
    #[serde(rename = "groupAvatar")]
    pub group_avatar: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    pub intro: String,
    #[serde(rename = "memberNum")]
    pub member_num: i32,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "ownerId_")]
    pub owner_id_: String,
    #[serde(rename = "qqChats")]
    pub qq_chats: Vec<QQChat>,
    #[serde(rename = "groupId_")]
    pub group_id_: String,
}

#[derive(Deserialize)]
pub struct QQChat {
    #[serde(rename = "chatName")]
    pub chat_name: String,
    #[serde(rename = "openGid")]
    pub open_gid: String,
}
