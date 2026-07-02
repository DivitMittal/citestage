use citestage_core::{Diagnosis, FailureClass, RepairPlan, StageStatus, StageTrace};

pub fn diagnose(trace: &StageTrace) -> Diagnosis {
    let failing = trace
        .stages
        .iter()
        .find(|stage| stage.status == StageStatus::Fail || stage.status == StageStatus::Partial);

    let Some(stage) = failing else {
        return Diagnosis {
            primary_failure: FailureClass::FactualityFailure,
            stage: "none".into(),
            evidence: vec!["all modeled stages passed; validate answer factuality manually".into()],
            repair_plan: RepairPlan {
                title: "Validate factuality outside the citation simulator".into(),
                steps: vec![
                    "Run human review against the source documents".into(),
                    "Add claim-level checks before publishing generated answers".into(),
                ],
            },
        };
    };

    let failure = failure_for_stage(&stage.stage);
    Diagnosis {
        primary_failure: failure.clone(),
        stage: stage.stage.clone(),
        evidence: stage.evidence.clone(),
        repair_plan: repair_plan(&failure),
    }
}

fn failure_for_stage(stage: &str) -> FailureClass {
    match stage {
        "crawl" => FailureClass::CrawlFailure,
        "parse" => FailureClass::ParseFailure,
        "index" => FailureClass::IndexFailure,
        "retrieve" => FailureClass::RetrievalFailure,
        "rerank" => FailureClass::RerankFailure,
        "synthesize" => FailureClass::SynthesisFailure,
        "cite" => FailureClass::CitationFailure,
        _ => FailureClass::FactualityFailure,
    }
}

fn repair_plan(failure: &FailureClass) -> RepairPlan {
    match failure {
        FailureClass::CrawlFailure => RepairPlan {
            title: "Make the project discoverable".into(),
            steps: vec![
                "Add a README.md at the repository root".into(),
                "Publish an llms.txt file that points engines to canonical docs".into(),
                "Ensure docs are linked from public package and repository metadata".into(),
            ],
        },
        FailureClass::ParseFailure => RepairPlan {
            title: "Make the README parse cleanly".into(),
            steps: vec![
                "Add a one-sentence definition near the top".into(),
                "Use descriptive Markdown headings instead of dense prose blocks".into(),
                "Move quickstart and use-cases before low-level topology details".into(),
            ],
        },
        FailureClass::IndexFailure => RepairPlan {
            title: "Improve indexable chunks".into(),
            steps: vec![
                "Repeat product and category terms in headings and first paragraphs".into(),
                "Split long sections into focused chunks with query-matching headings".into(),
                "Add concise summaries for major sections".into(),
            ],
        },
        FailureClass::RetrievalFailure => RepairPlan {
            title: "Align docs with likely user queries".into(),
            steps: vec![
                "Add use-case language that matches target search queries".into(),
                "Name competitor-adjacent concepts explicitly and accurately".into(),
                "Add examples for the queries where the target should appear".into(),
            ],
        },
        FailureClass::RerankFailure => RepairPlan {
            title: "Strengthen structural clarity signals".into(),
            steps: vec![
                "Add a clear definition in the first screenful".into(),
                "Add a use-cases section before implementation topology".into(),
                "Keep install instructions easy to identify".into(),
            ],
        },
        FailureClass::SynthesisFailure => RepairPlan {
            title: "Provide extractable answer material".into(),
            steps: vec![
                "Add short answer-shaped paragraphs under each key heading".into(),
                "Avoid hiding the value proposition in code blocks only".into(),
            ],
        },
        FailureClass::CitationFailure => RepairPlan {
            title: "Make citation assignment unambiguous".into(),
            steps: vec![
                "Ensure chunks include source-specific product names".into(),
                "Add canonical examples that are not duplicated from competitors".into(),
                "Add llms.txt with preferred citation URLs".into(),
            ],
        },
        FailureClass::FactualityFailure => RepairPlan {
            title: "Check claim support".into(),
            steps: vec![
                "Trace each generated claim to a source sentence".into(),
                "Remove unsupported comparison claims".into(),
            ],
        },
    }
}
