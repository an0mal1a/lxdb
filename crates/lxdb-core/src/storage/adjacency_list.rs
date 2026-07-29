use crate::ids::TokenId;

/// Points to the outgoing relations of every token.
#[derive(Debug, Default)]
pub struct AdjacencyList {
    entries: Vec<AdjacencyEntry>,
}

#[derive(Debug, Clone, Copy)]
pub struct AdjacencyEntry {
    offset: usize,
    count: usize,
}

impl AdjacencyEntry {
    pub const fn new(offset: usize, count: usize) -> Self {
        Self { offset, count }
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub const fn count(&self) -> usize {
        self.count
    }

    pub const fn end(&self) -> usize {
        self.offset + self.count
    }
}

impl AdjacencyList {
    pub fn new(entries: Vec<AdjacencyEntry>) -> Self {
        Self { entries }
    }

    pub fn get(&self, token_id: TokenId) -> Option<&AdjacencyEntry> {
        self.entries.get(token_id.value() as usize)
    }

    pub fn contains(&self, token_id: TokenId) -> bool {
        self.get(token_id).is_some()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
