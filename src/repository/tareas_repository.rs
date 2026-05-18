use sqlx::PgPool;
use crate::models::tareas::{Tarea, CrearTarea};

pub async fn obtener_todos(pool: &PgPool) -> Vec<Tarea> {
    sqlx::query_as::<_, Tarea>("SELECT * FROM Tareas")
        .fetch_all(pool).await.unwrap_or_default()
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Option<Tarea> {
    sqlx::query_as::<_, Tarea>(
        "SELECT * FROM Tareas WHERE id_tarea = $1"
    ).bind(id).fetch_optional(pool).await.unwrap_or(None)
}

pub async fn crear(pool: &PgPool, datos: CrearTarea) -> Tarea {
    sqlx::query_as::<_, Tarea>(
        "INSERT INTO Tareas (id_proyecto, descripcion, prioridad, estado)
         VALUES ($1, $2, $3, $4) RETURNING *"
    ).bind(datos.id_proyecto)
     .bind(datos.descripcion)
     .bind(datos.prioridad)
     .bind(datos.estado)
     .fetch_one(pool).await.unwrap()
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearTarea) -> Tarea {
    sqlx::query_as::<_, Tarea>(
        "UPDATE Tareas SET id_proyecto=$1, descripcion=$2, prioridad=$3, estado=$4
         WHERE id_tarea=$5 RETURNING *"
    ).bind(datos.id_proyecto)
     .bind(datos.descripcion)
     .bind(datos.prioridad)
     .bind(datos.estado)
     .bind(id)
     .fetch_one(pool).await.unwrap()
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    let resultado = sqlx::query(
        "DELETE FROM Tareas WHERE id_tarea = $1"
    ).bind(id).execute(pool).await;
    match resultado {
        Ok(r) => r.rows_affected() > 0,
        Err(e) => { println!("Error: {}", e); false }
    }
}