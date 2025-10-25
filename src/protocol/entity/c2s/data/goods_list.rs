use serde::Serialize;

#[derive(Serialize)]
pub struct C2SDataGoodsList {
    #[serde(rename = "groupId")]
    group_id: String,
    #[serde(rename = "filterStatus")]
    filter_status: i32,
    score: i32,
    limit: i32,
}

impl C2SDataGoodsList {
    pub fn new(group_id: String) -> Self {
        Self {
            group_id,
            filter_status: 0,
            score: -1,
            limit: 114514,
        }
    }
}
