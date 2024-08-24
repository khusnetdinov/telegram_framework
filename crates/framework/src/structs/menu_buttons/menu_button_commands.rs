use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::menu_button_commands::MenuButtonCommands as Remote;
use telegram_macros::FromRemoteStruct;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct)]
pub struct MenuButtonCommands {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
}