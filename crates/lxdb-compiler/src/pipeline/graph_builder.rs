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
