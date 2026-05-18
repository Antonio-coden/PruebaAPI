use sqlx::PgPool;
use crate::models::desarrolladores::{Desarrollador, CrearDesarrollador};
use crate::repository::desarrolladores_repository as repo;

pub async fn listar(pool: &PgPool) -> Vec<Desarrollador> {
    repo::obtener_todos(pool).await
}

pub async fn buscar(pool: &PgPool, id: i32) -> Option<Desarrollador> {
    repo::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, datos: CrearDesarrollador) -> Desarrollador {
    repo::crear(pool, datos).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearDesarrollador) -> Desarrollador {
    repo::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    repo::eliminar(pool, id).await
}