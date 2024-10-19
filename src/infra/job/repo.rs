use sqlx::{Pool, Postgres};
use serde_json::Value;
use crate::infra::job::model::Job;
use crate::infra::result::Result;
use crate::infra::uuid::{generate_uuid, UuidKind};

/// Salva um job no banco de dados, agendando o horário para executar, os parâmetros e o nome de job
/// O nome deve corresponder a um tipo de job predefinido que o sistema tenha a habilidade de executar
/// Consulte a lista de jobs implementados na camada `mod concrete;`
pub async fn incluir_job(
    pool: &Pool<Postgres>,
    id_empresa: String,
    // id_usuario: String,
    job_name: String,
    execute_at: time::OffsetDateTime,
    content: Value,
) -> Result<Job> {

    let novo_id = generate_uuid(UuidKind::V7);

    let rec = sqlx::query_as!(
        Job,
        "insert into job (id, description, context, execute_at, status, id_empresa )
	values( 
    $1, 
	$2, 
	$3::jsonb,
	$4,
	'pending',
    $5)
	returning id,
     description,
     execute_at,
     context,
     status",
        novo_id,
        job_name,
        content,
        execute_at,
        id_empresa,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}
