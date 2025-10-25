use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct S2CDataUserInfo {
    pub success: bool,
    #[serde(rename = "userinfo")]
    pub user_info: S2CDataUserInfoData,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    #[serde(rename = "needLogin")]
    pub need_login: bool,
    #[serde(rename = "adList")]
    pub ad_list: Vec<Value>,
    #[serde(rename = "groupListInfo")]
    pub group_list_info: GroupListInfo,
}

#[derive(Deserialize)]
pub struct GroupListInfo {
    #[serde(rename = "hasFoldGroup")]
    pub has_fold_group: bool,
    #[serde(rename = "hasOwnGroup")]
    pub has_own_group: bool,
}

#[derive(Deserialize)]
pub struct S2CDataUserInfoData {
    pub _id: String,
    pub avatar: String,
    pub birthday: String,
    pub city: String,
    pub country: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    pub gender: i64,
    #[serde(rename = "hasAuth")]
    pub has_auth: bool,
    #[serde(rename = "hasCloudStorage")]
    pub has_cloud_storage: bool,
    pub id: String,
    pub openid: String,
    pub phone: String,
    pub province: String,
    #[serde(rename = "qqName")]
    pub qq_name: String,
}
