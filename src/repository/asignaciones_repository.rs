use sqlx::PgPool;
use crate::models::asignaciones::{Asignacion, CrearAsignacion};

pub async fn obtener_todos(pool: &PgPool) -> Vec<Asignacion> {
    sqlx::query_as::<_, Asignacion>("SELECT * FROM Asignaciones")
        .fetch_all(pool).await.unwrap_or_default()
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Option<Asignacion> {
    sqlx::query_as::<_, Asignacion>(
        "SELECT * FROM Asignaciones WHERE id_asignacion = $1"
    ).bind(id).fetch_optional(pool).await.unwrap_or(None)
}

pub async fn crear(pool: &PgPool, datos: CrearAsignacion) -> Asignacion {
    sqlx::query_as::<_, Asignacion>(
        "INSERT INTO Asignaciones (id_tarea, id_desarrollador, horas_estimadas)
         VALUES ($1, $2, $3) RETURNING *"
    ).bind(datos.id_tarea)
     .bind(datos.id_desarrollador)
     .bind(datos.horas_estimadas)
     .fetch_one(pool).await.unwrap()
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearAsignacion) -> Asignacion {
    sqlx::query_as::<_, Asignacion>(
        "UPDATE Asignaciones SET id_tarea=$1, id_desarrollador=$2, horas_estimadas=$3
         WHERE id_asignacion=$4 RETURNING *"
    ).bind(datos.id_tarea)
     .bind(datos.id_desarrollador)
     .bind(datos.horas_estimadas)
     .bind(id)
     .fetch_one(pool).await.unwrap()
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    let resultado = sqlx::query(
        "DELETE FROM Asignaciones WHERE id_asignacion = $1"
    ).bind(id).execute(pool).await;
    match resultado {
        Ok(r) => r.rows_affected() > 0,
        Err(e) => { println!("Error: {}", e); false }
    }
}