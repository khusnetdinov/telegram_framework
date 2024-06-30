use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Emoji {
    Dice,
    Darts,
    Bowling,
    Basketball,
    Football,
    SlotMachine,
}

impl From<Emoji> for String {
    fn from(value: Emoji) -> String {
        match value {
            Emoji::Dice => "🎲",
            Emoji::Darts => "🎯",
            Emoji::Bowling => "🎳",
            Emoji::Basketball => "🏀",
            Emoji::Football => "⚽",
            Emoji::SlotMachine => "🎰",
        }
        .to_string()
    }
}
