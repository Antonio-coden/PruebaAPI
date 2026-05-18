use sqlx::PgPool;
use crate::models::desarrolladores::{Desarrollador, CrearDesarrollador};

pub async fn obtener_todos(pool: &PgPool) -> Vec<Desarrollador> {
    sqlx::query_as::<_, Desarrollador>("SELECT * FROM Desarrolladores")
        .fetch_all(pool).await.unwrap_or_default()
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Option<Desarrollador> {
    sqlx::query_as::<_, Desarrollador>(
        "SELECT * FROM Desarrolladores WHERE id_desarrollador = $1"
    ).bind(id).fetch_optional(pool).await.unwrap_or(None)
}

pub async fn crear(pool: &PgPool, datos: CrearDesarrollador) -> Desarrollador {
    sqlx::query_as::<_, Desarrollador>(
        "INSERT INTO Desarrolladores (nombre, rol_principal, nivel)
         VALUES ($1, $2, $3) RETURNING *"
    ).bind(datos.nombre)
     .bind(datos.rol_principal)
     .bind(datos.nivel)
     .fetch_one(pool).await.unwrap()
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearDesarrollador) -> Desarrollador {
    sqlx::query_as::<_, Desarrollador>(
        "UPDATE Desarrolladores SET nombre=$1, rol_principal=$2, nivel=$3
         WHERE id_desarrollador=$4 RETURNING *"
    ).bind(datos.nombre)
     .bind(datos.rol_principal)
     .bind(datos.nivel)
     .bind(id)
     .fetch_one(pool).await.unwrap()
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    let resultado = sqlx::query(
        "DELETE FROM Desarrolladores WHERE id_desarrollador = $1"
    ).bind(id).execute(pool).await;
    match resultado {
        Ok(r) => r.rows_affected() > 0,
        Err(e) => { println!("Error: {}", e); false }
    }
}