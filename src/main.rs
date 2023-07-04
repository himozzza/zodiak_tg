use teloxide::prelude::*;
use regex::Regex;

pub mod forecast;
pub mod arkan;
    
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
            let re: Regex = Regex::new("[0-9]{1,2}.[0-9]{1,2}.[0-9]{4}").unwrap();
            if re.is_match(msg.text().unwrap()) {
                let (arkan_one, arkan_two) = arkan::arkan(msg.text().unwrap().to_string()).await;
                let s: String = format!("Ваши арканы:\n1. {}.\n2. {}.", arkan_one, arkan_two);
                bot.send_message(msg.chat.id, s).await?;
            } else {
                let q: String = forecast::get_forecast(msg.text().unwrap().to_string()).await;
                bot.send_message(msg.chat.id, q).await?;
            }
            Ok(())
    })
    .await;
}

