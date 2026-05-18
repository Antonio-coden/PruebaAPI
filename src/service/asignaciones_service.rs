use sqlx::PgPool;
use crate::models::asignaciones::{Asignacion, CrearAsignacion};
use crate::repository::asignaciones_repository as repo;

pub async fn listar(pool: &PgPool) -> Vec<Asignacion> {
    repo::obtener_todos(pool).await
}

pub async fn buscar(pool: &PgPool, id: i32) -> Option<Asignacion> {
    repo::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, datos: CrearAsignacion) -> Asignacion {
    repo::crear(pool, datos).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearAsignacion) -> Asignacion {
    repo::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    repo::eliminar(pool, id).await
}