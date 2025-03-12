use sqlx::{PgPool, Error};
use chrono::Utc;
use crate::models::PaymentReminder;

pub async fn send_payment_reminders(
    pool: web::Data<PgPool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get_ref();

    let reminders = sqlx::query_as!(
        PaymentReminder,
        r#"
        SELECT id, contract_id, milestone_id, due_date, sent
        FROM payment_reminders
        WHERE sent = false AND due_date <= $1
        "#,
        Utc::now()
    )
    .fetch_all(conn)
    .await?;

    for reminder in reminders {
        if let Err(e) = send_reminder_email(&reminder.contract_id, &reminder.milestone_id).await {
            continue;
        }

        // Update payment reminder status
        sqlx::query!(
            r#"
            UPDATE payment_reminders
            SET sent = true
            WHERE id = $1
            "#,
            reminder.id
        )
        .execute(conn)
        .await?;
    }

    Ok(())
}



async fn send_reminder_email(contract_id: &u32, milestone_id: &u32) -> Result<(), Box<dyn std::error::Error>> {
    info!("Sending reminder email for contract_id: {} milestone_id: {}", contract_id, milestone_id);

    let domain = "your_domain.com";
    let api_key = "your_api_key";

    let mg = Mailgun::new(domain, api_key);

    let message = mg.message()
        .from("your_email@your_domain.com")
        .to("client_email@example.com")
        .cc("freelancer_email@example.com")
        .subject("Payment Reminder")
        .text(format!(
            "Payment reminder for contract {} milestone {}",
            contract_id, milestone_id
        ))
        .build();

    match mg.send(message) {
        Ok(_) => {
            info!("Reminder email sent successfully for contract_id: {} milestone_id: {}", contract_id, milestone_id);
            Ok(())
        },
        Err(e) => {
            error!("Failed to send reminder email for contract_id: {} milestone_id: {}: {:?}", contract_id, milestone_id, e);
            Err(Box::new(e))
        }
    }
}