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
        let re = Regex::new("[0-3]{0,1}[0-9]{1}.[0-1]{0,1}[0-2]{1}.[0-9]{4}").unwrap();
        let re_two = Regex::new("[а-яА-Я]*").unwrap();
        let q = match msg.text().unwrap().to_string() {
            x if re.is_match(&x.as_str()) => arkan::arkan(x.to_string()).await,
            x if re_two.is_match(x.as_str()) => forecast::get_forecast(x.to_string()).await,
            _ => todo!(),
        };

        bot.send_message(msg.chat.id, q).await?;
        
        Ok(())
        })
    .await;
}

