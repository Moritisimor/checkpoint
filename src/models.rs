use serde::Serialize;

// A collection of DTOs and other serializable models
#[derive(Serialize)]
pub struct GenericResponse {
    pub message: String
}
