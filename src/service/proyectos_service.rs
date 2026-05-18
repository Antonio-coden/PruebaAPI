use sqlx::PgPool;
use crate::models::proyectos::{Proyecto, CrearProyecto};
use crate::repository::proyectos_repository as repo;

pub async fn listar(pool: &PgPool) -> Vec<Proyecto> {
    repo::obtener_todos(pool).await
}

pub async fn buscar(pool: &PgPool, id: i32) -> Option<Proyecto> {
    repo::obtener_por_id(pool, id).await
}

pub async fn crear(pool: &PgPool, datos: CrearProyecto) -> Proyecto {
    repo::crear(pool, datos).await
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearProyecto) -> Proyecto {
    repo::actualizar(pool, id, datos).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    repo::eliminar(pool, id).await
}