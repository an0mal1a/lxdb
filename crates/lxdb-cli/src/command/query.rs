use std::path::Path;

use lxdb_engine::BinaryDatasetExt;

use crate::{command::open_dataset, error::CliError};

pub fn execute_query(dataset_path: &Path, token_text: &str) -> Result<(), CliError> {
    let dataset = open_dataset(dataset_path)?;

    let query = dataset.query();

    let Some(mut relations) = query.related_to(token_text)? else {
        println!("Token not found: {token_text}");

        return Ok(());
    };

    println!("{token_text}");

    if relations.len() == 0 {
        println!("└─ no outgoing relations");

        return Ok(());
    }

    while let Some(relation) = relations.next() {
        let relation = relation?;

        let line =
            format_relation(relations.len() == 0, relation.weight(), relation.target().text());

        println!("{line}");
    }

    Ok(())
}

fn format_relation(is_last: bool, weight: f32, target: &str) -> String {
    let branch = if is_last { "└─" } else { "├─" };

    format!("{branch} {weight:.3} → {target}",)
}

#[cfg(test)]
mod tests {
    use super::format_relation;

    #[test]
    fn formats_intermediate_relation() {
        let output = format_relation(false, 0.9, "language");

        assert_eq!(output, "├─ 0.900 → language",);
    }

    #[test]
    fn formats_last_relation() {
        let output = format_relation(true, 0.75, "compiler");

        assert_eq!(output, "└─ 0.750 → compiler",);
    }
}
