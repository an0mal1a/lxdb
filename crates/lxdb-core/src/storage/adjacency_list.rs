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
}

impl AdjacencyList {
    pub fn new(entries: Vec<AdjacencyEntry>) -> Self {
        Self { entries }
    }

    pub fn get(&self, token: usize) -> Option<&AdjacencyEntry> {
        self.entries.get(token)
    }
}