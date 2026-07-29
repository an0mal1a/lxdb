#[derive(Debug, Clone)]
pub struct RawRelation {
    pub source: String,
    pub target: String,
    pub weight: f32,
}