use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Tarea {
    pub id_tarea: i32,
    pub id_proyecto: Option<i32>,
    pub descripcion: String,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
}

#[derive(Deserialize)]
pub struct CrearTarea {
    pub id_proyecto: Option<i32>,
    pub descripcion: String,
    pub prioridad: Option<String>,
    pub estado: Option<String>,
}