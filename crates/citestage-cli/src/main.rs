use anyhow::{Context, Result};
use citestage_core::{Query, StageResult, StageStatus, StageTrace};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "citestage")]
#[command(about = "Diagnose citation failures in a deterministic local answer-engine pipeline.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a starter corpus manifest for a target README.")]
    Init {
        #[arg(long, help = "Path to the target project's README or source document")]
        target: PathBuf,
        #[arg(
            long,
            default_value = "corpus.yaml",
            help = "Path for the generated corpus manifest"
        )]
        output: PathBuf,
    },
    #[command(about = "Build corpus artifacts from local fixture documents.")]
    Corpus {
        #[command(subcommand)]
        command: CorpusCommands,
    },
    #[command(about = "Run the deterministic pipeline and write a JSON stage trace.")]
    Run {
        #[arg(long, help = "Query to evaluate against the controlled corpus")]
        query: String,
        #[arg(
            long,
            default_value = "corpus.yaml",
            help = "Path to the corpus manifest"
        )]
        corpus: PathBuf,
        #[arg(
            long,
            default_value = "stage-trace.json",
            help = "Path for the generated JSON stage trace"
        )]
        output: PathBuf,
    },
    #[command(about = "Run the pipeline and write an evidence-backed Markdown diagnosis.")]
    Explain {
        #[arg(long, help = "Query to evaluate against the controlled corpus")]
        query: String,
        #[arg(long, help = "Override the corpus target id for this diagnosis")]
        target: Option<String>,
        #[arg(
            long,
            default_value = "corpus.yaml",
            help = "Path to the corpus manifest"
        )]
        corpus: PathBuf,
        #[arg(
            long,
            default_value = "diagnosis.md",
            help = "Path for the generated Markdown diagnosis report"
        )]
        output: PathBuf,
    },
    #[command(about = "Print the repair plan from a previously generated stage trace.")]
    PatchPlan {
        #[arg(
            long,
            default_value = "stage-trace.json",
            help = "Path to a JSON stage trace with diagnosis data"
        )]
        trace: PathBuf,
    },
    #[command(about = "Compare two diagnosis traces at the primary-failure summary level.")]
    Compare {
        #[arg(long, help = "Path to the baseline JSON stage trace")]
        before: PathBuf,
        #[arg(long, help = "Path to the follow-up JSON stage trace")]
        after: PathBuf,
    },
}

#[derive(Subcommand)]
enum CorpusCommands {
    #[command(
        about = "Ingest local target, competitor, and distractor documents into corpus JSON."
    )]
    Build {
        #[arg(long, help = "Path to the target project's README or source document")]
        target: PathBuf,
        #[arg(
            long = "competitor",
            help = "Path to a competitor document; repeat for multiple competitors"
        )]
        competitors: Vec<PathBuf>,
        #[arg(
            long = "distractor",
            help = "Path to a distractor document; repeat for multiple distractors"
        )]
        distractors: Vec<PathBuf>,
        #[arg(
            long,
            default_value = "corpus.json",
            help = "Path for the generated corpus JSON"
        )]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { target, output } => {
            citestage_corpus::init_manifest(target, output)?;
        }
        Commands::Corpus { command } => match command {
            CorpusCommands::Build {
                target,
                competitors,
                distractors,
                output,
            } => {
                let corpus =
                    citestage_corpus::build_from_paths(target, &competitors, &distractors)?;
                let json = serde_json::to_string_pretty(&corpus).context("serialize corpus")?;
                fs::write(output, json).context("write corpus JSON")?;
            }
        },
        Commands::Run {
            query,
            corpus,
            output,
        } => {
            let trace = run_pipeline(&query, &corpus, None)?;
            let json = serde_json::to_string_pretty(&trace).context("serialize stage trace")?;
            fs::write(output, json).context("write stage trace")?;
        }
        Commands::Explain {
            query,
            target,
            corpus,
            output,
        } => {
            let trace = run_pipeline(&query, &corpus, target.as_deref())?;
            let report = citestage_report::render_markdown(&trace);
            fs::write(output, report).context("write diagnosis report")?;
        }
        Commands::PatchPlan { trace } => {
            let trace = read_trace(trace)?;
            let diagnosis = trace
                .diagnosis
                .context("trace does not contain a diagnosis")?;
            println!("# {}", diagnosis.repair_plan.title);
            for step in diagnosis.repair_plan.steps {
                println!("- {}", step);
            }
        }
        Commands::Compare { before, after } => {
            let before = read_trace(before)?;
            let after = read_trace(after)?;
            println!("# Compare diagnosis runs");
            println!("before: {}", summary(&before));
            println!("after: {}", summary(&after));
        }
    }

    Ok(())
}

fn run_pipeline(
    query_text: &str,
    corpus_path: &PathBuf,
    target_override: Option<&str>,
) -> Result<StageTrace> {
    let mut corpus = citestage_corpus::build_from_manifest(corpus_path)?;
    if let Some(target) = target_override {
        corpus.target_id = target.to_string();
    }

    let target_id = corpus.target_id.clone();
    let mut stages = Vec::new();
    stages.push(crawl_stage(&corpus));

    let parsed = citestage_parse::parse_documents(&corpus.documents);
    stages.push(parse_stage(&parsed, &target_id));

    let chunks = citestage_index::chunks_from_parsed(&parsed);
    let index = citestage_index::Bm25Index::new(chunks);
    stages.push(index_stage(&index, &target_id));

    let retrieved = citestage_retrieve::retrieve(&index, query_text, 8);
    stages.push(citestage_retrieve::retrieval_stage(&retrieved, &target_id));

    let reranked = citestage_rerank::rerank(query_text, &retrieved, &parsed);
    stages.push(citestage_rerank::rerank_stage(
        &retrieved, &reranked, &target_id,
    ));

    let answer = citestage_generate::synthesize(query_text, &reranked, 3);
    stages.push(citestage_generate::synthesize_stage(&answer, &target_id));
    stages.push(citestage_generate::citation_stage(&answer, &target_id));

    let mut trace = StageTrace {
        query: Query {
            text: query_text.to_string(),
            target_id: target_id.clone(),
        },
        target: target_id,
        stages,
        cited_sources: answer.cited_sources,
        diagnosis: None,
    };
    trace.diagnosis = Some(citestage_diagnose::diagnose(&trace));
    Ok(trace)
}

fn crawl_stage(corpus: &citestage_core::Corpus) -> StageResult {
    if corpus.target().is_none() {
        return StageResult::with_rank(
            "crawl",
            StageStatus::Fail,
            None,
            vec!["target document was not loaded into the corpus".into()],
        );
    }

    StageResult::pass(
        "crawl",
        vec![format!("loaded {} documents", corpus.documents.len())],
    )
}

fn parse_stage(parsed: &[citestage_core::ParsedDocument], target_id: &str) -> StageResult {
    let Some(target) = parsed.iter().find(|doc| doc.document.id == target_id) else {
        return StageResult::with_rank(
            "parse",
            StageStatus::Fail,
            None,
            vec!["target was unavailable after parsing".into()],
        );
    };

    let status = if target.features.has_top_definition {
        StageStatus::Pass
    } else {
        StageStatus::Partial
    };
    let mut evidence = vec![format!(
        "parsed {} sections; clarity score {:.2}",
        target.sections.len(),
        target.features.clarity_score
    )];
    if !target.features.has_top_definition {
        evidence.push("no one-sentence definition detected near the top".into());
    }

    StageResult::with_rank("parse", status, Some(1), evidence)
}

fn index_stage(index: &citestage_index::Bm25Index, target_id: &str) -> StageResult {
    let target_chunks = index
        .chunks()
        .iter()
        .filter(|chunk| chunk.document_id == target_id)
        .count();
    if target_chunks == 0 {
        return StageResult::with_rank(
            "index",
            StageStatus::Fail,
            None,
            vec!["target produced no indexable chunks".into()],
        );
    }

    StageResult::with_rank(
        "index",
        StageStatus::Pass,
        Some(1),
        vec![format!(
            "target produced {} indexable chunks",
            target_chunks
        )],
    )
}

fn read_trace(path: PathBuf) -> Result<StageTrace> {
    let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&text).context("parse stage trace JSON")
}

fn summary(trace: &StageTrace) -> String {
    trace
        .diagnosis
        .as_ref()
        .map(|diagnosis| format!("{:?} at {}", diagnosis.primary_failure, diagnosis.stage))
        .unwrap_or_else(|| "no diagnosis".into())
}
