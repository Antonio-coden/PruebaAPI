use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Desarrollador {
    pub id_desarrollador: i32,
    pub nombre: String,
    pub rol_principal: Option<String>,
    pub nivel: Option<String>,
}

#[derive(Deserialize)]
pub struct CrearDesarrollador {
    pub nombre: String,
    pub rol_principal: Option<String>,
    pub nivel: Option<String>,
}