use crate::enums::message_result::MessageResult;
use crate::structs::games::game_high_score::GameHighScore;
use crate::structs::games::options::Options as GameOptions;
use crate::structs::options::Options;
use crate::structs::updates::message::Message;

#[async_trait::async_trait]
pub trait Game {
    async fn get_game_high_scores(
        &self,
        user_id: i64,
        chat_id: i64,
        options: Option<GameOptions>,
    ) -> Result<Vec<GameHighScore>, Box<dyn std::error::Error>>;

    async fn send_game(
        &self,
        chat_id: i64,
        game_short_name: String,
        options: Option<Options>,
    ) -> Result<Message, Box<dyn std::error::Error>>;

    async fn set_game_score(
        &self,
        user_id: i64,
        score: u64,
        options: Option<GameOptions>,
    ) -> Result<MessageResult, Box<dyn std::error::Error>>;
}
