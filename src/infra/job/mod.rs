pub mod model;
pub mod service;
// pub mod view;
pub mod repo;
pub mod concrete;


use std::time::Duration;
use tokio::time::interval;
use sqlx::PgPool;

pub async fn job_scheduler(pool: PgPool) {
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        let jobs = sqlx::query!(
            r#"
            SELECT id, description, context FROM job
            WHERE execute_at <= now() AND status = 'pending'
            "#
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch jobs");

        for job in jobs {
            println!("Executing job: {}", job.description);

            // Marcar o job como concluído
            sqlx::query!(
                "UPDATE job SET status = 'completed' WHERE id = $1",
                job.id
            )
            .execute(&pool)
            .await
            .expect("Failed to update job status");
        }
    }
}

