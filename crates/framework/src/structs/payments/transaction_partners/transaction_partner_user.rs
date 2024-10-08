use crate::enums::paid_media::PaidMedia;
use crate::structs::user::User;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::transaction_partner_user::TransactionPartnerUser as Remote;
use telegram_macros::{FromRemoteStruct, IntoRemoteStruct};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteStruct, IntoRemoteStruct)]
pub struct TransactionPartnerUser {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub user: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<PaidMedia>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,
}
