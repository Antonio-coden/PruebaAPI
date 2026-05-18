use axum::{Router, routing::{get, post, put, delete}, extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::clientes_corp::CrearClienteCorp;
use crate::service::clientes_corp_service as service;

pub fn clientes_corp_router(pool: PgPool) -> Router {
    Router::new()
        .route("/clientes", get(listar).post(crear))
        .route("/clientes/{id}", get(buscar).put(actualizar).delete(eliminar))
        .with_state(pool)
}

async fn listar(State(pool): State<PgPool>) -> Json<serde_json::Value> {
    Json(serde_json::json!(service::listar(&pool).await))
}

async fn buscar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<serde_json::Value> {
    Json(serde_json::json!(service::buscar(&pool, id).await))
}

async fn crear(State(pool): State<PgPool>, Json(datos): Json<CrearClienteCorp>) -> Json<serde_json::Value> {
    Json(serde_json::json!(service::crear(&pool, datos).await))
}

async fn actualizar(State(pool): State<PgPool>, Path(id): Path<i32>, Json(datos): Json<CrearClienteCorp>) -> Json<serde_json::Value> {
    Json(serde_json::json!(service::actualizar(&pool, id, datos).await))
}

async fn eliminar(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<serde_json::Value> {
    let eliminado = service::eliminar(&pool, id).await;
    if eliminado {
        Json(serde_json::json!({"mensaje": "Cliente eliminado correctamente"}))
    } else {
        Json(serde_json::json!({"error": "No se pudo eliminar, verifique el id o registros relacionados"}))
    }
}