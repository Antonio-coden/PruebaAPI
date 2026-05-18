mod models;
mod repository;
mod service;
mod controller;
mod config;

use controller::clientes_corp_controller::clientes_corp_router;
use controller::proyectos_controller::proyectos_router;
use controller::desarrolladores_controller::desarrolladores_router;
use controller::tareas_controller::tareas_router;
use controller::asignaciones_controller::asignaciones_router;
use config::config::crear_pool;

#[tokio::main]
async fn main() {
    let direccion = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(direccion)
        .await
        .expect("No se pudo enlazar el puerto 3000");

    println!("Servidor escuchando en http://{direccion}");

    let pool = crear_pool()
        .await
        .expect("No se pudo conectar a la base de datos");

    axum::serve(listener, unificar_routers(pool))
        .await
        .expect("Error al iniciar el servidor");
}

fn unificar_routers(pool: sqlx::PgPool) -> axum::Router {
    clientes_corp_router(pool.clone())
        .merge(proyectos_router(pool.clone()))
        .merge(desarrolladores_router(pool.clone()))
        .merge(tareas_router(pool.clone()))
        .merge(asignaciones_router(pool.clone()))
}