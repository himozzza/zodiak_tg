use std::{thread, time};
use chrono::prelude::*;
use teloxide::prelude::*;
use regex::Regex;

pub mod forecast;
pub mod arkan;
pub mod db_update;


fn main() {
    thread::spawn(||{
        loop {
            if Local::now().hour().to_string() == "21".to_string() {
                db_update::db_update();
                thread::sleep(time::Duration::from_secs(3600));
            } else {
                thread::sleep(time::Duration::from_secs(1800));
            }
        }
    });
    bot()
}

#[tokio::main]
async fn bot() {
    pretty_env_logger::init();
    let bot: Bot = Bot::from_env();
        teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        
        let answer: String = match msg.text().unwrap().to_string() {
            x if Regex::new("[0-9]{0,1}[0-9]{1}.[0-9]{0,1}[0-9]{1}.[0-9]{1,4}")
                                .unwrap().is_match(&x.as_str()) => 
                                    arkan::arkan(x.to_string()).await,
            x if Regex::new(".*[а-яА-Я].*").unwrap()
                                .is_match(x.as_str()) => 
                                    forecast::get_forecast(msg.clone()),
            _ => String::from("Возможно Вы допустили ошибку."),
        };

        bot.send_message(msg.chat.id, answer).await?;
        
        Ok(())
        })
    .await;
}
