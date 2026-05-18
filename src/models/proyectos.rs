use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Proyecto {
    pub id_proyecto: i32,
    pub nombre_proyecto: String,
    pub fecha_inicio: Option<chrono::NaiveDate>,
    pub id_cliente: Option<i32>,
}

#[derive(Deserialize)]
pub struct CrearProyecto {
    pub nombre_proyecto: String,
    pub fecha_inicio: Option<chrono::NaiveDate>,
    pub id_cliente: Option<i32>,
}