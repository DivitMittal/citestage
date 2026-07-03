use citestage_core::{Diagnosis, StageTrace};

pub fn render_markdown(trace: &StageTrace) -> String {
    let mut output = String::new();
    output.push_str("# CiteStage Diagnosis Report\n\n");
    output.push_str(&format!("**Query:** `{}`\n\n", trace.query.text));
    output.push_str(&format!("**Target:** `{}`\n\n", trace.target));
    output.push_str("## Pipeline trace\n\n");
    output.push_str("| Stage | Status | Target rank | Evidence |\n");
    output.push_str("| --- | --- | --- | --- |\n");

    for stage in &trace.stages {
        let rank = stage
            .target_rank
            .map(|rank| rank.to_string())
            .unwrap_or_else(|| "—".into());
        output.push_str(&format!(
            "| {} | {:?} | {} | {} |\n",
            stage.stage,
            stage.status,
            rank,
            stage.evidence.join("; ").replace('|', "\\|")
        ));
    }

    output.push('\n');
    if let Some(diagnosis) = &trace.diagnosis {
        append_diagnosis(&mut output, diagnosis);
    } else {
        output.push_str("## Primary failure\n\nNo diagnosis was attached to this trace.\n");
    }

    output
}

fn append_diagnosis(output: &mut String, diagnosis: &Diagnosis) {
    output.push_str("## Primary failure\n\n");
    output.push_str(&format!(
        "**{:?}** at stage `{}`.\n\n",
        diagnosis.primary_failure, diagnosis.stage
    ));
    output.push_str("### Evidence\n\n");
    for item in &diagnosis.evidence {
        output.push_str(&format!("- {}\n", item));
    }
    output.push_str("\n### Suggested repairs\n\n");
    output.push_str(&format!("**{}**\n\n", diagnosis.repair_plan.title));
    for step in &diagnosis.repair_plan.steps {
        output.push_str(&format!("- {}\n", step));
    }
}
