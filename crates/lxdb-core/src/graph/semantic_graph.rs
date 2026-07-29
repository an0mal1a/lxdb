use crate::model::{Relation, Token};

/// Immutable semantic graph.
///
/// The graph owns every token and every relation.
///
/// Algorithms are implemented by higher level crates.
#[derive(Debug, Default)]
pub struct SemanticGraph {
    pub tokens: Vec<Token>,
    pub relations: Vec<Relation>,
}