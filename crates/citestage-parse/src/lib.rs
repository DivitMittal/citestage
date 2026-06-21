use citestage_core::{tokenize, Document, ParsedDocument, Section, StructuralFeatures};
use regex::Regex;

pub fn parse_document(document: &Document) -> ParsedDocument {
    let sections = split_sections(&document.content);
    let summary = first_tokens(&document.content, 40);
    let features = structural_features(&document.content, &sections);

    ParsedDocument {
        document: document.clone(),
        sections,
        summary,
        features,
    }
}

pub fn parse_documents(documents: &[Document]) -> Vec<ParsedDocument> {
    documents.iter().map(parse_document).collect()
}

fn split_sections(content: &str) -> Vec<Section> {
    let heading_re = Regex::new(r"^(#{1,6})\s+(.+)\s*$").expect("valid heading regex");
    let mut sections = Vec::new();
    let mut current_heading = String::from("Overview");
    let mut current_level = 1;
    let mut current_text = String::new();

    for line in content.lines() {
        if let Some(captures) = heading_re.captures(line) {
            push_section(
                &mut sections,
                &current_heading,
                current_level,
                &current_text,
            );
            current_heading = captures[2].trim().to_string();
            current_level = captures[1].len();
            current_text.clear();
            continue;
        }

        current_text.push_str(line);
        current_text.push('\n');
    }

    push_section(
        &mut sections,
        &current_heading,
        current_level,
        &current_text,
    );

    if sections.is_empty() {
        sections.push(Section {
            heading: "Overview".into(),
            level: 1,
            text: content.to_string(),
        });
    }

    sections
}

fn push_section(sections: &mut Vec<Section>, heading: &str, level: usize, text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }

    sections.push(Section {
        heading: heading.to_string(),
        level,
        text: trimmed.to_string(),
    });
}

fn structural_features(content: &str, sections: &[Section]) -> StructuralFeatures {
    let top = content.lines().take(12).collect::<Vec<_>>().join(" ");
    let has_top_definition = has_definition_sentence(&top);
    let max_heading_depth = sections
        .iter()
        .map(|section| section.level)
        .max()
        .unwrap_or(1);
    let install_section_position = sections.iter().position(|section| {
        let heading = section.heading.to_lowercase();
        heading.contains("install") || heading.contains("quickstart") || heading.contains("setup")
    });
    let use_case_term_count = count_use_case_terms(content);
    let clarity_score = clarity_score(
        has_top_definition,
        install_section_position,
        use_case_term_count,
    );

    StructuralFeatures {
        has_top_definition,
        max_heading_depth,
        install_section_position,
        use_case_term_count,
        clarity_score,
    }
}

fn has_definition_sentence(top: &str) -> bool {
    let top = top.to_lowercase();
    [
        " is a ",
        " is an ",
        " helps ",
        " lets ",
        " provides ",
        "debugger",
    ]
    .iter()
    .any(|needle| top.contains(needle))
}

fn count_use_case_terms(content: &str) -> usize {
    let lower = content.to_lowercase();
    [
        "use case",
        "example",
        "when to use",
        "for teams",
        "helps",
        "workflow",
    ]
    .iter()
    .filter(|term| lower.contains(**term))
    .count()
}

fn clarity_score(
    has_definition: bool,
    install_position: Option<usize>,
    use_case_terms: usize,
) -> f32 {
    let mut score: f32 = 0.2;
    if has_definition {
        score += 0.4;
    }
    if install_position.is_some_and(|position| position <= 3) {
        score += 0.2;
    }
    score += (use_case_terms.min(4) as f32) * 0.05;
    score.min(1.0)
}

fn first_tokens(content: &str, count: usize) -> String {
    tokenize(content)
        .into_iter()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use citestage_core::SourceKind;

    #[test]
    fn detects_top_definition() {
        let doc = Document {
            id: "target".into(),
            title: "CiteStage".into(),
            path: "README.md".into(),
            source_kind: SourceKind::Target,
            content: "# CiteStage\n\nCiteStage is a debugger for citation failures.\n\n## Install\nRun it.".into(),
        };
        let parsed = parse_document(&doc);
        assert!(parsed.features.has_top_definition);
        assert!(parsed.features.clarity_score > 0.7);
    }
}
