use sqlx::PgPool;
use crate::models::clientes_corp::{ClienteCorp, CrearClienteCorp};
use crate::repository::clientes_corp_repository as repo;

pub async fn listar(pool: &PgPool) -> Vec<ClienteCorp> {
    repo::obtener_todos(pool).await
}

pub async fn buscar(pool: &PgPool, id: i32) -> Option<ClienteCorp> {
    repo::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, datos: CrearClienteCorp) -> ClienteCorp {
    repo::crear(pool, datos).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearClienteCorp) -> ClienteCorp {
    repo::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    repo::eliminar(pool, id).await
}