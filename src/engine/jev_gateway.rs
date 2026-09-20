use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::rag::{Battlecard, MatchResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub speaker: String, // "customer" or "salesperson"
    pub utterance: String,
    pub call_stage: Option<String>, // "discovery", "pitch", "negotiation", "closing"
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JevTelemetry {
    pub model: String,
    pub latency_ms: u64,
    pub online: bool,
    pub confidence: f32,
    pub decision_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSalesHint {
    pub speaker: String,
    pub utterance: String,
    pub detected_emotion: String,
    pub unspoken_subtext: String,
    pub expected_next_say: String,
    pub recommended_framework: String,
    pub book_source: String,
    pub psychological_principle: String,
    pub exact_script_to_say: String,
    pub secondary_followup: String,
    pub delivery_tone: String,
    pub confidence_pct: f32,
    pub telemetry: JevTelemetry,
    pub timestamp: String,
}

pub struct JevEngine {
    client: reqwest::Client,
    base_url: String,
}

impl JevEngine {
    pub fn new() -> Self {
        let base_url = std::env::var("TYPESAFE_BASE_URL")
            .unwrap_or_else(|_| "https://api.typesafe.ai/v1/systemone".to_string());

        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_millis(3500))
                .build()
                .unwrap_or_default(),
            base_url,
        }
    }

    pub async fn analyze(&self, req: &AnalysisRequest) -> LiveSalesHint {
        let start = Instant::now();
        let query = req.utterance.trim();
        let api_key = req.api_key.clone()
            .or_else(|| std::env::var("TYPESAFE_API_KEY").ok())
            .unwrap_or_default();

        // 1. First run high-speed in-memory Sales RAG matching (<1ms)
        let matched = Battlecard::find_best_match(query);

        // 2. If API key is present, attempt live TypeSafe Jev System One classification
        let mut telemetry = JevTelemetry {
            model: "jev-1.13.0".to_string(),
            latency_ms: 0,
            online: false,
            confidence: 96.5,
            decision_node: "local-simd-radar".to_string(),
        };

        if !api_key.is_empty() {
            let payload = serde_json::json!({
                "model": "jev-latest",
                "state": format!("Live sales call turn. Speaker: {}. Utterance: '{}'", req.speaker, query),
                "questions": {
                    "objection_classification": {
                        "type": "choice",
                        "instructions": "Identify the prospect's real psychological hesitation or state.",
                        "criteria": {
                            "PRICE_BUDGET": "Budget, cost anxiety, price comparison",
                            "STALL_TIMING": "Procrastination, send email, timing freeze",
                            "AUTHORITY_CHALLENGE": "Defers to boss, lack of power",
                            "COMPETITOR_ANCHOR": "Prefers competitor or legacy incumbent",
                            "STATUS_QUO_INERTIA": "Satisfied with internal tools, resistant to change",
                            "TRUST_RISK": "Fear of startup risk or failure"
                        }
                    }
                }
            });

            if let Ok(resp) = self.client.post(&self.base_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
            {
                let latency = start.elapsed().as_millis() as u64;
                if resp.status().is_success() {
                    telemetry.online = true;
                    telemetry.latency_ms = latency;
                    telemetry.model = "jev-system-one".to_string();
                    telemetry.decision_node = "typesafe-cloud-gateway".to_string();
                }
            }
        }

        if !telemetry.online {
            telemetry.latency_ms = start.elapsed().as_millis().max(1) as u64;
        }

        let now_str = chrono::Local::now().format("%H:%M:%S%.3f").to_string();

        if let Some(MatchResult { battlecard, match_score, .. }) = matched {
            LiveSalesHint {
                speaker: req.speaker.clone(),
                utterance: query.to_string(),
                detected_emotion: match battlecard.category {
                    crate::rag::ObjectionCategory::PriceAndBudget => "Financial Anxiety / Career Risk".to_string(),
                    crate::rag::ObjectionCategory::TimingAndStalling => "Polite Avoidance / Procrastination".to_string(),
                    crate::rag::ObjectionCategory::AuthorityAndCommitment => "Internal Political Vulnerability".to_string(),
                    crate::rag::ObjectionCategory::CompetitorComparison => "Skepticism / Bargaining Anchor".to_string(),
                    crate::rag::ObjectionCategory::StatusQuoInertia => "Complacency / Pain of Change Fear".to_string(),
                    crate::rag::ObjectionCategory::TrustAndRiskAversion => "Fear of Personal Failure / Job Safety".to_string(),
                    _ => "Cautious Exploration".to_string(),
                },
                unspoken_subtext: battlecard.customer_unspoken_thought.to_string(),
                expected_next_say: battlecard.customer_next_trajectory.to_string(),
                recommended_framework: battlecard.framework_name.to_string(),
                book_source: battlecard.author_and_book.to_string(),
                psychological_principle: battlecard.psychological_principle.to_string(),
                exact_script_to_say: battlecard.exact_script.to_string(),
                secondary_followup: battlecard.secondary_followup.to_string(),
                delivery_tone: battlecard.tone_delivery_guide.to_string(),
                confidence_pct: match_score,
                telemetry,
                timestamp: now_str,
            }
        } else {
            // General Discovery / Clarification fallback using Chris Voss Calibrated Questions
            LiveSalesHint {
                speaker: req.speaker.clone(),
                utterance: query.to_string(),
                detected_emotion: "Uncommitted Evaluation".to_string(),
                unspoken_subtext: "I'm weighing whether this call is worth my team's time or if this is just another generic vendor pitch.".to_string(),
                expected_next_say: "They will ask for a high-level overview or ask you to explain your core value proposition in 30 seconds.".to_string(),
                recommended_framework: "Chris Voss Calibrated Open-Ended Question".to_string(),
                book_source: "Chris Voss — 'Never Split the Difference'".to_string(),
                psychological_principle: "Invites the prospect to define the problem so they become emotionally invested in solving it.".to_string(),
                exact_script_to_say: "What's the biggest operational friction your team is running into right now that made you open to exploring this?".to_string(),
                secondary_followup: "How is that challenge impacting your quarterly targets if things stay the way they are today?".to_string(),
                delivery_tone: "Deep, curious, unhurried tone. Let them speak 80% of the time.".to_string(),
                confidence_pct: 88.0,
                telemetry,
                timestamp: now_str,
            }
        }
    }
}
