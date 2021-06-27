use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let request_id = Uuid::new_v4();
    log::info!(
        "r-Id {}Adding '{}' '{}' as a new subscriber.",
        request_id
        form.email,
        form.name
    );
    log::info!("Saving new subscriber details in the database");
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // There is a bit of ceremony here to get our hands on a &PgConnection.
    // web::Data<Arc<PgConnection>> is equivalent to Arc<Arc<PgConnection>>
    // Therefore connection.get_ref() returns a &Arc<PgConnection>
    // which we can then deref to a &PgConnection.
    // We could have avoided the double Arc wrapping using .app_data()
    // instead of .data() in src/startup.rs - we'll get to it later!
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        log::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    log::info!("New subscriber details have been saved");
    Ok(HttpResponse::Ok().finish())
}
