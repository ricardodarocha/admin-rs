use sqlx::PgPool;

#[allow(unused)]
async fn notify(pool: &PgPool, s: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
SELECT pg_notify(chan, payload)
FROM (VALUES ('chan0', $1)) v(chan, payload)
"#,
    )
    .bind(s)
    .execute(pool)
    .await?;

    Ok(())
}