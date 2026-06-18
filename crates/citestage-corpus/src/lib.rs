use anyhow::{anyhow, Context, Result};
use citestage_core::{Corpus, Document, SourceKind};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusManifest {
    pub target: ManifestEntry,
    #[serde(default)]
    pub competitors: Vec<ManifestEntry>,
    #[serde(default)]
    pub distractors: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub id: String,
    pub title: String,
    pub path: String,
}

pub fn init_manifest(target: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<()> {
    let target = target.as_ref();
    let entry = entry_for_path(target, "target", "Target Project");
    let manifest = CorpusManifest {
        target: entry,
        competitors: Vec::new(),
        distractors: Vec::new(),
    };
    write_manifest(&manifest, output)
}

pub fn write_manifest(manifest: &CorpusManifest, output: impl AsRef<Path>) -> Result<()> {
    let yaml = serde_yaml::to_string(manifest).context("serialize corpus manifest")?;
    fs::write(output.as_ref(), yaml).with_context(|| format!("write {}", output.as_ref().display()))
}

pub fn load_manifest(path: impl AsRef<Path>) -> Result<CorpusManifest> {
    let text = fs::read_to_string(path.as_ref())
        .with_context(|| format!("read manifest {}", path.as_ref().display()))?;
    serde_yaml::from_str(&text).context("parse corpus manifest")
}

pub fn build_from_manifest(path: impl AsRef<Path>) -> Result<Corpus> {
    let manifest_path = path.as_ref();
    let base = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let manifest = load_manifest(manifest_path)?;
    build_from_entries(base, manifest)
}

pub fn build_from_paths(
    target: impl AsRef<Path>,
    competitors: &[PathBuf],
    distractors: &[PathBuf],
) -> Result<Corpus> {
    let manifest = CorpusManifest {
        target: entry_for_path(target.as_ref(), "target", "Target Project"),
        competitors: competitors
            .iter()
            .enumerate()
            .map(|(index, path)| {
                entry_for_path(path, &format!("competitor-{}", index + 1), "Competitor")
            })
            .collect(),
        distractors: distractors
            .iter()
            .enumerate()
            .map(|(index, path)| {
                entry_for_path(path, &format!("distractor-{}", index + 1), "Distractor")
            })
            .collect(),
    };
    build_from_entries(Path::new("."), manifest)
}

fn build_from_entries(base: &Path, manifest: CorpusManifest) -> Result<Corpus> {
    let target_id = manifest.target.id.clone();
    let mut documents = Vec::new();
    documents.push(read_entry(base, &manifest.target, SourceKind::Target)?);

    for entry in &manifest.competitors {
        documents.push(read_entry(base, entry, SourceKind::Competitor)?);
    }

    for entry in &manifest.distractors {
        documents.push(read_entry(base, entry, SourceKind::Distractor)?);
    }

    if documents.is_empty() {
        return Err(anyhow!("corpus contains no documents"));
    }

    Ok(Corpus {
        target_id,
        documents,
    })
}

fn read_entry(base: &Path, entry: &ManifestEntry, source_kind: SourceKind) -> Result<Document> {
    let path = resolve_document_path(base, &entry.path)?;
    let content = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    Ok(Document {
        id: entry.id.clone(),
        title: entry.title.clone(),
        path: path.display().to_string(),
        source_kind,
        content,
    })
}

fn resolve_document_path(base: &Path, raw: &str) -> Result<PathBuf> {
    let path = Path::new(raw);
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    };

    if joined.is_file() {
        return Ok(joined);
    }

    if !joined.is_dir() {
        return Err(anyhow!("{} is not a file or directory", joined.display()));
    }

    for name in ["README.md", "readme.md", "Readme.md"] {
        let candidate = joined.join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    Err(anyhow!("{} has no README.md", joined.display()))
}

fn entry_for_path(path: &Path, id: &str, fallback_title: &str) -> ManifestEntry {
    let title = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or(fallback_title)
        .replace(['-', '_'], " ");

    ManifestEntry {
        id: id.to_string(),
        title,
        path: path.display().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_serializes() {
        let manifest = CorpusManifest {
            target: ManifestEntry {
                id: "target".into(),
                title: "Target".into(),
                path: "README.md".into(),
            },
            competitors: Vec::new(),
            distractors: Vec::new(),
        };
        let yaml = serde_yaml::to_string(&manifest).unwrap();
        assert!(yaml.contains("target"));
    }
}
