use crate::dashboard::model::*;
use crate::infra::result::Result;
// use crate::infra::uuid::{generate_uuid, UuidKind};
use sqlx::{Pool, Postgres};

pub async fn dash_for_user(pool: &Pool<Postgres>, user: String) -> Result<Vec<DashboardCard>> {
    let rec = sqlx::query_as!(
        DashboardCard,
        "
        select * from dashboard
where id_usuario = $1
and id_grupo_dashboard in ('main', 'registro')
        ",user,
    )
    .fetch_all(pool)
    .await?;

    Ok(rec)
}

pub async fn clientes_for_user(pool: &Pool<Postgres>, user: String) -> Result<Vec<DashboardClientes>> {
    let rec = sqlx::query_as!(
        DashboardClientes,
        "
        select c.id, c.nome, coalesce(c.avatar, 'https://icons.veryicon.com/png/o/miscellaneous/standard-general-linear-icon/dashboard-80.png') as avatar, max(coalesce(p.created, c.created)) as created from pessoa c
        left join pedido p on c.id = p.id_cliente 
        where 1=1 or p.id_usuario = $1
        group by 1, 2 order by 2 desc", user,
    )
    .fetch_all(pool)
    .await?;

    Ok(rec)
}
