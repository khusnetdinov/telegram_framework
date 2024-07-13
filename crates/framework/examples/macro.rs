use telegram_bots_api::api::structs::order_info::OrderInfo;
use telegram_bots_api::api::structs::successful_payment::SuccessfulPayment as Remote;
use telegram_macros::FromRemote;

#[derive(FromRemote)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
