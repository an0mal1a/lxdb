use std::collections::HashMap;

use lxdb_core::{
    graph::SemanticGraph,
    ids::{RelationId, TokenId},
    model::{Relation, Token, Weight},
    storage::{AdjacencyEntry, AdjacencyList},
};

use crate::{error::CompilerError, pipeline::parser::ParseResult};

/// Converts validated raw data into a semantic graph.
#[derive(Debug, Default)]
pub struct GraphBuilder;

impl GraphBuilder {
    pub fn build(&self, result: ParseResult) -> Result<SemanticGraph, CompilerError> {
        let mut token_ids = HashMap::with_capacity(result.tokens.len());
        let mut tokens = Vec::with_capacity(result.tokens.len());

        for (index, raw_token) in result.tokens.into_iter().enumerate() {
            let token_id = TokenId::new(index as u32);
            let text = raw_token.text.trim().to_owned();

            if token_ids.insert(text.clone(), token_id).is_some() {
                return Err(CompilerError::DuplicateToken(text));
            }

            tokens.push(Token::new(token_id, text.into_boxed_str()));
        }

        let mut relations = Vec::with_capacity(result.relations.len());

        for raw_relation in result.relations {
            let source_text = raw_relation.source.trim();
            let target_text = raw_relation.target.trim();

            let source = token_ids
                .get(source_text)
                .copied()
                .ok_or_else(|| CompilerError::UnknownToken(source_text.to_owned()))?;

            let target = token_ids
                .get(target_text)
                .copied()
                .ok_or_else(|| CompilerError::UnknownToken(target_text.to_owned()))?;

            let weight =
                Weight::new(raw_relation.weight).map_err(|_| CompilerError::InvalidWeight)?;

            relations.push(Relation::new(RelationId::new(0), source, target, weight));
        }

        relations.sort_unstable_by_key(|relation| relation.source());

        for (index, relation) in relations.iter_mut().enumerate() {
            *relation = Relation::new(
                RelationId::new(index as u32),
                relation.source(),
                relation.target(),
                relation.weight(),
            );
        }

        let adjacency = build_adjacency(tokens.len(), &relations);

        Ok(SemanticGraph { tokens, relations, adjacency })
    }
}

fn build_adjacency(token_count: usize, relations: &[Relation]) -> AdjacencyList {
    let mut entries = Vec::with_capacity(token_count);
    let mut relation_index = 0;

    for token_index in 0..token_count {
        let start = relation_index;

        while relation_index < relations.len()
            && relations[relation_index].source().value() as usize == token_index
        {
            relation_index += 1;
        }

        entries.push(AdjacencyEntry::new(start, relation_index - start));
    }

    AdjacencyList::new(entries)
}

#[cfg(test)]
mod tests {
    use lxdb_core::{ids::TokenId, traversal::GraphTraversal};

    use super::GraphBuilder;
    use crate::{
        model::{RawRelation, RawToken},
        pipeline::parser::ParseResult,
    };

    #[test]
    fn builds_tokens_relations_and_adjacency() {
        let input = ParseResult {
            tokens: vec![
                RawToken { text: "rust".to_owned() },
                RawToken { text: "systems".to_owned() },
                RawToken { text: "memory".to_owned() },
            ],
            relations: vec![
                RawRelation {
                    source: "rust".to_owned(),
                    target: "systems".to_owned(),
                    weight: 0.9,
                },
                RawRelation { source: "rust".to_owned(), target: "memory".to_owned(), weight: 0.8 },
                RawRelation {
                    source: "systems".to_owned(),
                    target: "memory".to_owned(),
                    weight: 0.7,
                },
            ],
        };

        let graph = GraphBuilder.build(input).expect("graph construction should succeed");

        assert!(graph.contains(TokenId::new(0)));
        assert!(graph.contains(TokenId::new(1)));
        assert!(graph.contains(TokenId::new(2)));
        assert!(!graph.contains(TokenId::new(3)));

        assert_eq!(graph.outgoing(TokenId::new(0)).len(), 2);
        assert_eq!(graph.outgoing(TokenId::new(1)).len(), 1);
        assert!(graph.outgoing(TokenId::new(2)).is_empty());
        assert!(graph.outgoing(TokenId::new(99)).is_empty());
    }

    #[test]
    fn rejects_relations_with_unknown_tokens() {
        let input = ParseResult {
            tokens: vec![RawToken { text: "rust".to_owned() }],
            relations: vec![RawRelation {
                source: "rust".to_owned(),
                target: "unknown".to_owned(),
                weight: 0.5,
            }],
        };

        let result = GraphBuilder.build(input);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_tokens() {
        let input = ParseResult {
            tokens: vec![
                RawToken { text: "rust".to_owned() },
                RawToken { text: "rust".to_owned() },
            ],
            relations: vec![],
        };

        let result = GraphBuilder.build(input);

        assert!(result.is_err());
    }
}
