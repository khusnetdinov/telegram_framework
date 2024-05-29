use telegram_bots_api::api::structs::poll::Poll as Inner;
use telegram_macros::{DerefInner, FromInner};

#[derive(Debug, DerefInner, FromInner)]
pub struct Poll {
    inner: Inner,
}
