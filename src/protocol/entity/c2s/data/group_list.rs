use serde::Serialize;

#[derive(Serialize)]
pub struct C2SDataGetGroupList {
    #[serde(rename = "userId")]
    user_id: String,
    next: &'static str,
    limit: i32,
    #[serde(rename = "type")]
    _type: &'static str,
}

impl C2SDataGetGroupList {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            next: "",
            limit: 114514,
            _type: "unfold",
        }
    }
}
