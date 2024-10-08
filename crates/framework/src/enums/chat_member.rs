use crate::structs::chats::chat_members::chat_member_administrator::ChatMemberAdministrator;
use crate::structs::chats::chat_members::chat_member_banned::ChatMemberBanned;
use crate::structs::chats::chat_members::chat_member_left::ChatMemberLeft;
use crate::structs::chats::chat_members::chat_member_member::ChatMemberMember;
use crate::structs::chats::chat_members::chat_member_owner::ChatMemberOwner;
use crate::structs::chats::chat_members::chat_member_restricted::ChatMemberRestricted;
use serde::{Deserialize, Serialize};
use telegram_bots_api::api::enums::chat_member::ChatMember as Remote;
use telegram_macros::{FromRemoteEnum, IntoRemoteEnum};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRemoteEnum, IntoRemoteEnum)]
#[serde(untagged)]
pub enum ChatMember {
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    Banned(ChatMemberBanned),
}

impl Default for ChatMember {
    fn default() -> Self {
        Self::Owner(ChatMemberOwner {
            ..Default::default()
        })
    }
}
