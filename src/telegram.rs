use anyhow::{Context, Result};
use reqwest::Client;
use serde::Serialize;

pub struct TelegramApi<'a> {
    bot_token: &'a str,
}

#[derive(Serialize)]
struct SendMessageRequest<'a> {
    chat_id: i64,
    text: &'a str,
}

impl<'a> TelegramApi<'a> {
    pub fn new(bot_token: &'a str) -> TelegramApi<'a> {
        TelegramApi { bot_token }
    }

    pub async fn send_message(
        &self,
        chat_id: i64,
        message: &str,
    ) -> Result<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let client = Client::new();
        client
            .post(&url)
            .json(&SendMessageRequest {
                chat_id,
                text: message,
            })
            .send()
            .await?
            .error_for_status()
            .context("Failed to send telegram message")?;

        Ok(())
    }
}