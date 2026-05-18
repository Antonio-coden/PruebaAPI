use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ClienteCorp {
    pub id_cliente: i32,
    pub nombre_empresa: String,
    pub pais: Option<String>,
    pub contacto_principal: Option<String>,
}

#[derive(Deserialize)]
pub struct CrearClienteCorp {
    pub nombre_empresa: String,
    pub pais: Option<String>,
    pub contacto_principal: Option<String>,
}