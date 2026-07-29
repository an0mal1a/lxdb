/// Binary representation of a semantic relation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RelationRecord {
    pub source: u32,
    pub target: u32,
    pub weight: f32,
}
