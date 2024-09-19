use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::background_fill_solid::BackgroundFillSolid as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct BackgroundFillSolid {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub color: u32,
}
