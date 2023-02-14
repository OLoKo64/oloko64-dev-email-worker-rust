use crate::{responses::UserError, utils::EnvVars};

pub struct Telegram;

impl Telegram {
    pub async fn send_notification<T, U>(subject: U, message: T) -> Result<String, UserError>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let bot_token = EnvVars::get_telegram_bot_token()?;
        let chat_id = EnvVars::get_telegram_chat_id()?;

        let response = reqwest::Client::new()
            .post(format!(
                "https://api.telegram.org/bot{bot_token}/sendMessage"
            ))
            .json(&serde_json::json!({
                "chat_id": chat_id,
                "text": format!("Subject: {}\n\n{}", subject.as_ref(), message.as_ref()),
            }))
            .send()
            .await
            .map_err(|err| UserError::InternalServerError {
                message: "Error while sending Telegram notification".to_string(),
                error: err.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(UserError::InternalServerError {
                message: format!(
                    "Error sending Telegram notification, request status {}",
                    response.status()
                ),
                error: response
                    .text()
                    .await
                    .map_err(|err| UserError::InternalServerError {
                        message: "Could not convert response text".to_string(),
                        error: err.to_string(),
                    })?,
            });
        }

        response
            .text()
            .await
            .map_err(|err| UserError::InternalServerError {
                message: "Could not convert response text".to_string(),
                error: err.to_string(),
            })
    }
}
