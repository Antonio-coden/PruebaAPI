use sqlx::PgPool;
use crate::models::clientes_corp::{ClienteCorp, CrearClienteCorp};

pub async fn obtener_todos(pool: &PgPool) -> Vec<ClienteCorp> {
    sqlx::query_as::<_, ClienteCorp>("SELECT * FROM Clientes_Corp")
        .fetch_all(pool).await.unwrap_or_default()
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Option<ClienteCorp> {
    sqlx::query_as::<_, ClienteCorp>(
        "SELECT * FROM Clientes_Corp WHERE id_cliente = $1"
    ).bind(id).fetch_optional(pool).await.unwrap_or(None)
}

pub async fn crear(pool: &PgPool, datos: CrearClienteCorp) -> ClienteCorp {
    sqlx::query_as::<_, ClienteCorp>(
        "INSERT INTO Clientes_Corp (nombre_empresa, pais, contacto_principal)
         VALUES ($1, $2, $3) RETURNING *"
    ).bind(datos.nombre_empresa)
     .bind(datos.pais)
     .bind(datos.contacto_principal)
     .fetch_one(pool).await.unwrap()
}

pub async fn actualizar(pool: &PgPool, id: i32, datos: CrearClienteCorp) -> ClienteCorp {
    sqlx::query_as::<_, ClienteCorp>(
        "UPDATE Clientes_Corp SET nombre_empresa=$1, pais=$2, contacto_principal=$3
         WHERE id_cliente=$4 RETURNING *"
    ).bind(datos.nombre_empresa)
     .bind(datos.pais)
     .bind(datos.contacto_principal)
     .bind(id)
     .fetch_one(pool).await.unwrap()
}

pub async fn eliminar(pool: &PgPool, id: i32) -> bool {
    let resultado = sqlx::query(
        "DELETE FROM Clientes_Corp WHERE id_cliente = $1"
    ).bind(id).execute(pool).await;
    match resultado {
        Ok(r) => r.rows_affected() > 0,
        Err(e) => { println!("Error: {}", e); false }
    }
}