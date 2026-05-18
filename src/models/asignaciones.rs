use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Asignacion {
    pub id_asignacion: i32,
    pub id_tarea: Option<i32>,
    pub id_desarrollador: Option<i32>,
    pub horas_estimadas: Option<i32>,
}

#[derive(Deserialize)]
pub struct CrearAsignacion {
    pub id_tarea: Option<i32>,
    pub id_desarrollador: Option<i32>,
    pub horas_estimadas: Option<i32>,
}