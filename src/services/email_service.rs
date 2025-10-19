
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
}

impl EmailService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_username = env::var("SMTP_USERNAME")?;
        let smtp_password = env::var("SMTP_PASSWORD")?;
        let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let from_email = env::var("FROM_EMAIL")?;

        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::relay(&smtp_server)?
            .credentials(creds)
            .build();

        Ok(Self { mailer, from_email })
    }

    pub fn send_welcome_email(&self, to_email: &str, username: &str) -> Result<(), Box<dyn std::error::Error>> {
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center; border-radius: 10px 10px 0 0; }}
                    .content {{ background: #f9f9f9; padding: 30px; border-radius: 0 0 10px 10px; }}
                    .button {{ display: inline-block; padding: 12px 30px; background: #667eea; color: white; text-decoration: none; border-radius: 5px; margin-top: 20px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Welcome to Workanda! 🚀</h1>
                    </div>
                    <div class="content">
                        <h2>Hi {},</h2>
                        <p>Thank you for joining Workanda - the future of freelancing!</p>
                        <p>We're excited to have you on board. Here's what you can do next:</p>
                        <ul>
                            <li>Complete your profile</li>
                            <li>Browse available jobs</li>
                            <li>Connect with clients or freelancers</li>
                            <li>Start earning securely with our escrow system</li>
                        </ul>
                        <a href="https://workanda.com/dashboard" class="button">Get Started</a>
                        <p style="margin-top: 30px; color: #666;">Need help? Contact us at support@workanda.com</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            username
        );

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to_email.parse()?)
            .subject("Welcome to Workanda!")
            .multipart(MultiPart::alternative_plain_html(
                format!("Welcome to Workanda, {}!", username),
                html_body,
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_job_notification_email(&self, to_email: &str, job_title: &str, job_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background: #667eea; color: white; padding: 20px; text-align: center; }}
                    .content {{ background: #f9f9f9; padding: 30px; }}
                    .button {{ display: inline-block; padding: 12px 30px; background: #667eea; color: white; text-decoration: none; border-radius: 5px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h2>🔔 New Job Opportunity</h2>
                    </div>
                    <div class="content">
                        <h3>{}</h3>
                        <p>A new job matching your skills has been posted!</p>
                        <a href="https://workanda.com/jobs/{}" class="button">View Job</a>
                    </div>
                </div>
            </body>
            </html>
            "#,
            job_title, job_id
        );

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to_email.parse()?)
            .subject(format!("New Job: {}", job_title))
            .multipart(MultiPart::alternative_plain_html(
                format!("New job opportunity: {}", job_title),
                html_body,
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_payment_confirmation(&self, to_email: &str, amount: f64, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background: #10b981; color: white; padding: 20px; text-align: center; }}
                    .content {{ background: #f9f9f9; padding: 30px; }}
                    .amount {{ font-size: 32px; color: #10b981; font-weight: bold; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h2>💰 Payment Received</h2>
                    </div>
                    <div class="content">
                        <p>Great news! You've received a payment.</p>
                        <p class="amount">${:.2}</p>
                        <p><strong>Project:</strong> {}</p>
                        <p>The funds have been credited to your Workanda account.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            amount, project_name
        );

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to_email.parse()?)
            .subject("Payment Received")
            .multipart(MultiPart::alternative_plain_html(
                format!("You received ${:.2} for {}", amount, project_name),
                html_body,
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub fn send_milestone_completion(&self, to_email: &str, milestone_name: &str, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background: #8b5cf6; color: white; padding: 20px; text-align: center; }}
                    .content {{ background: #f9f9f9; padding: 30px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h2>✅ Milestone Completed</h2>
                    </div>
                    <div class="content">
                        <h3>{}</h3>
                        <p><strong>Project:</strong> {}</p>
                        <p>Congratulations on completing this milestone! Keep up the great work.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            milestone_name, project_name
        );

        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to_email.parse()?)
            .subject(format!("Milestone Completed: {}", milestone_name))
            .multipart(MultiPart::alternative_plain_html(
                format!("Milestone completed: {}", milestone_name),
                html_body,
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }
}
