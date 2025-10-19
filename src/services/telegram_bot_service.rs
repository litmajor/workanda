
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub chat_id: String,
    pub text: String,
    pub parse_mode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TelegramResponse {
    pub ok: bool,
    pub result: Option<serde_json::Value>,
}

pub struct TelegramBotService {
    client: Client,
    bot_token: String,
    base_url: String,
}

impl TelegramBotService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bot_token = env::var("TELEGRAM_BOT_TOKEN")
            .map_err(|_| "TELEGRAM_BOT_TOKEN not set in environment")?;
        
        Ok(Self {
            client: Client::new(),
            bot_token: bot_token.clone(),
            base_url: format!("https://api.telegram.org/bot{}", bot_token),
        })
    }

    pub async fn send_message(&self, message: TelegramMessage) -> Result<TelegramResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/sendMessage", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await?;

        let telegram_response: TelegramResponse = response.json().await?;
        Ok(telegram_response)
    }

    pub async fn send_job_notification(&self, chat_id: &str, job_title: &str, job_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let message = TelegramMessage {
            chat_id: chat_id.to_string(),
            text: format!(
                "🔔 *New Job Alert*\n\n*{}*\n\nJob ID: {}\n\nCheck it out on Workanda!",
                job_title, job_id
            ),
            parse_mode: Some("Markdown".to_string()),
        };

        self.send_message(message).await?;
        Ok(())
    }

    pub async fn send_proposal_notification(&self, chat_id: &str, freelancer_name: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
        let message = TelegramMessage {
            chat_id: chat_id.to_string(),
            text: format!(
                "💼 *New Proposal Received*\n\n*From:* {}\n*Amount:* ${:.2}\n\nView on Workanda!",
                freelancer_name, amount
            ),
            parse_mode: Some("Markdown".to_string()),
        };

        self.send_message(message).await?;
        Ok(())
    }

    pub async fn send_payment_notification(&self, chat_id: &str, amount: f64, status: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = TelegramMessage {
            chat_id: chat_id.to_string(),
            text: format!(
                "💰 *Payment Update*\n\n*Amount:* ${:.2}\n*Status:* {}\n\nCheck your dashboard!",
                amount, status
            ),
            parse_mode: Some("Markdown".to_string()),
        };

        self.send_message(message).await?;
        Ok(())
    }

    pub async fn send_milestone_notification(&self, chat_id: &str, milestone_name: &str, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = TelegramMessage {
            chat_id: chat_id.to_string(),
            text: format!(
                "✅ *Milestone Completed*\n\n*Project:* {}\n*Milestone:* {}\n\nGreat work!",
                project_name, milestone_name
            ),
            parse_mode: Some("Markdown".to_string()),
        };

        self.send_message(message).await?;
        Ok(())
    }
}
