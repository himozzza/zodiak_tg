use teloxide::prelude::*;
use regex::Regex;

pub mod forecast;
pub mod arkan;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let answer: String = match msg.text().unwrap().to_string() {
            x if Regex::new("[0-9]{0,1}[0-9]{1}.[0-9]{0,1}[0-9]{1}.[0-9]{1,4}")
                                .unwrap().is_match(&x.as_str()) => 
                                    arkan::arkan(x.to_string()).await,
            x if Regex::new(".*[а-яА-Я].*").unwrap()
                                .is_match(x.as_str()) => 
                                    forecast::get_forecast(x.to_string()).await,
            _ => String::from("Возможно Вы допустили ошибку."),
        };

        bot.send_message(msg.chat.id, answer).await?;
        
        Ok(())
        })
    .await;
}

