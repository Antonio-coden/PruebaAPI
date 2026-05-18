use sqlx::PgPool;
use crate::models::proyectos::{Proyecto, CrearProyecto};

pub async fn obtener_todos(pool: &PgPool) -> Vec<Proyecto> {
    sqlx::query_as::<_, Proyecto>("SELECT * FROM Proyectos")
        .fetch_all(pool).await.unwrap_or_default()
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Option<Proyecto> {
    sqlx::query_as::<_, Proyecto>(
        "SELECT * FROM Proyectos WHERE id_proyecto = $1"
    ).bind(id).fetch_optional(pool).await.unwrap_or(None)
}

pub async fn crear(pool: &PgPool, datos: CrearProyecto) -> Proyecto {
    sqlx::query_as::<_, Proyecto>(
        "INSERT INTO Proyectos (nombre_proyecto, fecha_inicio, id_cliente)
         VALUES ($1, $2, $3) RETURNING *"
    ).bind(datos.nombre_proyecto)
     .bind(datos.fecha_inicio)
     .bind(datos.id_cliente)
     .fetch_one(pool).await.unwrap()
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearProyecto) -> Proyecto {
    sqlx::query_as::<_, Proyecto>(
        "UPDATE Proyectos SET nombre_proyecto=$1, fecha_inicio=$2, id_cliente=$3
         WHERE id_proyecto=$4 RETURNING *"
    ).bind(datos.nombre_proyecto)
     .bind(datos.fecha_inicio)
     .bind(datos.id_cliente)
     .bind(id)
     .fetch_one(pool).await.unwrap()
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    let resultado = sqlx::query(
        "DELETE FROM Proyectos WHERE id_proyecto = $1"
    ).bind(id).execute(pool).await;
    match resultado {
        Ok(r) => r.rows_affected() > 0,
        Err(e) => { println!("Error: {}", e); false }
    }
}