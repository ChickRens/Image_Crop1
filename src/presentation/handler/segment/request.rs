#[derive(serde::Deserialize)]
pub struct SegmentRequest {
    pub session_id: String,
    pub point: (u16, u16),
    pub is_foreground: bool,
}