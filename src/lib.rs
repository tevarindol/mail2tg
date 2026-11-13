mod telegram;

use anyhow::{
    Context,
    Result
};
use telegram::TelegramApi;
use mail_parser::MessageParser;
use worker::{
    Env,
    ForwardableEmailMessage,
    event,
};

#[event(email)]
async fn email(
    message: ForwardableEmailMessage,
    env: Env,
    _ctx: worker::Context,
) -> Result<()> {
    let bot_token = env.secret("BOT_TOKEN")
        .context("Failed to get telegram bot token")?
        .to_string();
    let chat_id: i64 = env.secret("CHAT_ID")
        .context("Failed to get telegram chat id")?
        .to_string()
        .parse()
        .context("Failed to parse chat_id")?;

    let telegram_api = TelegramApi::new(&bot_token);
    let parser = MessageParser::new();

    let from = message.from();
    let to = message.to();
    let raw = message.raw_bytes().await?;

    let email = parser.parse(&raw)
        .context("Failed to parse email")?;
    let subject = email.subject()
        .unwrap_or("(no subject)");
    let body = email.body_text(0)
        .unwrap_or_default();

    telegram_api.send_message(
        chat_id,
        &format!(
            "Ви отримали лист\n
            З пошти {from}\n
            На вашу пошту {to}\n
            \n
            Заголовок: {subject}\n
            Вміст листа:\n
            {body}\n"
        )
    ).await?;

    Ok(())
}