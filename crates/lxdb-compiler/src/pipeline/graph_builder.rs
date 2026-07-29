use std::collections::HashMap;

use lxdb_core::{
    graph::SemanticGraph,
    ids::{RelationId, TokenId},
    model::{Relation, Token, Weight},
    storage::AdjacencyList,
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

        for (index, raw_relation) in result.relations.into_iter().enumerate() {
            let source = token_ids
                .get(raw_relation.source.trim())
                .copied()
                .ok_or_else(|| CompilerError::UnknownToken(raw_relation.source.clone()))?;

            let target = token_ids
                .get(raw_relation.target.trim())
                .copied()
                .ok_or_else(|| CompilerError::UnknownToken(raw_relation.target.clone()))?;

            let weight =
                Weight::new(raw_relation.weight).map_err(|_| CompilerError::InvalidWeight)?;

            relations.push(Relation::new(RelationId::new(index as u32), source, target, weight));
        }

        relations.sort_unstable_by_key(|relation| relation.source());

        Ok(SemanticGraph { tokens, relations, adjacency: AdjacencyList::default() })
    }
}
