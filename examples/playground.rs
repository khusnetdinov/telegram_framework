use telegram_framework::structs::bots_api::BotsApi;

fn main() {
    let bots_api = BotsApi::new();

    bots_api.pooling(move |updates| {
        println!("#{:#?}", updates);
    });
}
