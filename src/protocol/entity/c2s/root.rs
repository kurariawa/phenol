use serde::Serialize;

use crate::protocol::ticket::Ticket;

#[derive(Serialize)]
pub struct C2SRoot {
    action: &'static str,
    ticket: Ticket,
    #[serde(rename = "dataVersion")]
    data_version: &'static str,
    function_name: String,
    request_data: String,
    env: &'static str,
    #[serde(rename = "loginType")]
    login_type: &'static str,
}

impl C2SRoot {
    pub fn new(ticket: Ticket, function_name: String, request_data: String) -> Self {
        Self {
            action: "functions.invokeFunction",
            ticket,
            data_version: "2019-08-16", // magic string
            function_name,
            request_data,
            env: "pro-2gis2vsrb5a3b312", // id for the app
            login_type: "QQ-MINI",       // qq mini app type
        }
    }
}
