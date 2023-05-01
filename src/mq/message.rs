#[derive(Debug, serde::Deserialize)]
pub struct Message {
    pub trace_id: String,
    pub body: String,
}