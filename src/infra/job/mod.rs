pub mod model;
pub mod service;
// pub mod view;
pub mod repo;
pub mod concrete;

use concrete::SendEmail;
use log::info;
use model::{Job, Jober};
use std::time::Duration;
use tokio::time::interval;
use sqlx::PgPool;

use crate::config::database::autorecover;

pub async fn job_scheduler(pool: PgPool) {
    let mut interval = interval(Duration::from_secs(10));

    loop {
        interval.tick().await;

        let jobs = sqlx::query_as!(
            Job, 
            r#"
            SELECT id, description, execute_at, context, status FROM job
            WHERE execute_at <= now() AND status = 'pending'
            "#
        )
        .fetch_all(&autorecover(&pool).await)
        .await
        .expect("Failed to fetch jobs");

        println!("🔎 searching jobs");

        for job in jobs {
            println!("🔨found job: {} to execute at {:?}", job.description, job.execute_at);
            info!("▶ executing... {:?} ", job.description);

            let job_clone = job.clone();
            let pool_clone = pool.clone();
            
            // Avisa que já colocou o job na fila; isso previne que outros jobs façam uma chamada duplicada
                sqlx::query!(
                    "UPDATE job SET status = 'processing...' WHERE id = $1",
                    job.clone().id
                ).execute(&autorecover(&pool).await)
                .await
                .expect("Failed to update job status");

            // Executa o job em uma nova tarefa para não bloquear o loop
            tokio::task::spawn(async move {
                match job_clone.description.as_str() {
                    "✉" => { SendEmail::run(job_clone.clone()).await; },
                    _ => {},
                }

                // Marcar o job como concluído
                sqlx::query!(
                    "UPDATE job SET status = 'completed' WHERE id = $1",
                    job_clone.id
                )
                .execute(&pool_clone)
                .await
                .expect("Failed to update job status");

                info!("✅ job completed: {:?}", job_clone.description);
            });
        }
    }
}

