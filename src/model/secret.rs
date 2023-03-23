use std::time::Duration;

#[derive(Debug)]
pub struct Secret {
    pub id: i32,
    pub data: String,
    pub ttl: Duration,
}