use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct SecretRequest {
    pub data: String,
    pub ttl: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SecretReponse {
    pub uuid: Uuid,
    pub data: String,
    pub ttl: usize,
}
