use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::giveaway_completed::GiveawayCompleted as Remote;
use telegram_bots_api::api::structs::message::Message as IncomingMessage;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(
    Debug, Clone, Default, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct,
)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<Box<Self>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
}

impl From<IncomingMessage> for GiveawayCompleted {
    fn from(remote: IncomingMessage) -> Self {
        let IncomingMessage {
            giveaway_completed, ..
        } = remote;

        Self::from(giveaway_completed.unwrap())
    }
}
