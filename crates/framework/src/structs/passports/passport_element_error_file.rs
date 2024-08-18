use serde::{Deserialize, Serialize};
use telegram_bots_api::api::structs::passport_element_error_file::PassportElementErrorFile as Remote;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassportElementErrorFile {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub source: String,
    pub message: String,
    pub file_hash: String,
}

impl From<Remote> for PassportElementErrorFile {
    fn from(remote: Remote) -> Self {
        Self {
            kind: remote.kind,
            source: remote.source,
            message: remote.message,
            file_hash: remote.file_hash,
        }
    }
}

impl From<PassportElementErrorFile> for Remote {
    fn from(value: PassportElementErrorFile) -> Self {
        Self {
            kind: value.kind,
            source: value.source,
            message: value.message,
            file_hash: value.file_hash,
        }
    }
}
