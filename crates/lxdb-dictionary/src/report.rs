use std::{collections::BTreeMap, time::Duration};

#[derive(Debug, Clone, Default)]
pub struct BuildReport {
    pub entries_read: u64,
    pub entries_invalid: u64,
    pub entries_accepted: u64,
    pub entries_rejected: u64,
    pub unique_lemmas: u64,
    pub surface_forms: u64,
    pub senses: u64,
    pub relations: u64,
    pub duplicate_entries: u64,
    pub connected_tokens: u64,
    pub isolated_tokens: u64,
    pub relation_types: BTreeMap<String, u64>,
    pub relation_sources: BTreeMap<String, u64>,
    pub phases: BTreeMap<String, Duration>,
}
