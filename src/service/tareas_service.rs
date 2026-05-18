use sqlx::PgPool;
use crate::models::tareas::{Tarea, CrearTarea};
use crate::repository::tareas_repository as repo;

pub async fn listar(pool: &PgPool) -> Vec<Tarea> {
    repo::obtener_todos(pool).await
}

pub async fn buscar(pool: &PgPool, id: i32) -> Option<Tarea> {
    repo::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, datos: CrearTarea) -> Tarea {
    repo::crear(pool, datos).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearTarea) -> Tarea {
    repo::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    repo::eliminar(pool, id).await
}