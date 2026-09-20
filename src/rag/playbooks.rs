use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectionCategory {
    PriceAndBudget,
    TimingAndStalling,
    AuthorityAndCommitment,
    CompetitorComparison,
    StatusQuoInertia,
    TrustAndRiskAversion,
    FeatureDeficit,
    GeneralDiscovery,
}

#[derive(Debug, Clone, Serialize)]
pub struct Battlecard {
    pub id: &'static str,
    pub category: ObjectionCategory,
    pub trigger_patterns: &'static [&'static str],
    pub customer_unspoken_thought: &'static str,
    pub customer_next_trajectory: &'static str,
    pub framework_name: &'static str,
    pub author_and_book: &'static str,
    pub psychological_principle: &'static str,
    pub exact_script: &'static str,
    pub secondary_followup: &'static str,
    pub tone_delivery_guide: &'static str,
}

pub static BATTLECARDS: &[Battlecard] = &[
    // 1. PRICE SHOCK / BUDGET
    Battlecard {
        id: "voss_price_shock",
        category: ObjectionCategory::PriceAndBudget,
        trigger_patterns: &[
            "too expensive",
            "price is high",
            "over our budget",
            "can't afford",
            "cost too much",
            "sticker shock",
            "out of our price range",
        ],
        customer_unspoken_thought: "I like this, but I'm terrified my CFO or finance team will chew me out for spending capital right now. I need an ironclad ROI shield.",
        customer_next_trajectory: "They will demand a 30-50% blanket discount or defer the decision to next fiscal quarter.",
        framework_name: "Tactical Empathy & Calibrated Labeling",
        author_and_book: "Chris Voss — 'Never Split the Difference'",
        psychological_principle: "Disarms defensive amygdala activation by vocalizing their unspoken fear before they defend it.",
        exact_script: "It sounds like you're under immense pressure from finance to protect cash flow this quarter, and any new commitment feels like a career risk.",
        secondary_followup: "How does this cost compare to the revenue your team is burning every month by running the current manual setup?",
        tone_delivery_guide: "Late-night FM DJ voice: calm, slow, completely devoid of defensive or apologetic pitch.",
    },

    // 2. DISCOUNT PRESSURE
    Battlecard {
        id: "voss_discount_pressure",
        category: ObjectionCategory::PriceAndBudget,
        trigger_patterns: &[
            "can you give us a discount",
            "any discount",
            "what's your best price",
            "lower the price",
            "sharpen the pencil",
            "give me a deal",
        ],
        customer_unspoken_thought: "I'm testing your posture to see if you have pricing integrity. If you fold instantly, I'll know you were overcharging me.",
        customer_next_trajectory: "They will treat your price as a loose guideline and demand concession after concession without giving anything up.",
        framework_name: "The Ackerman Model & Calibrated 'How' Question",
        author_and_book: "Chris Voss — 'Never Split the Difference'",
        psychological_principle: "Forces the buyer to solve your pricing problem instead of allowing them to pressure you into unilateral concessions.",
        exact_script: "How am I supposed to do that without gutting the dedicated support tier and rollout engineers your team specifically asked for?",
        secondary_followup: "We can adjust scope or payment terms, but if we drop the price arbitrarily, what deliverables are you comfortable cutting?",
        tone_delivery_guide: "Gently curious, completely non-confrontational. Make them reflect on the tradeoff.",
    },

    // 3. COMPETITOR LOWER PRICE
    Battlecard {
        id: "spin_competitor_cheaper",
        category: ObjectionCategory::CompetitorComparison,
        trigger_patterns: &[
            "competitor is cheaper",
            "other vendor is half the price",
            "we got a quote from",
            "cheaper alternative",
            "why are you more expensive than",
        ],
        customer_unspoken_thought: "I'm anchoring to the bottom-feeder in your space to squeeze you, even though I know their product is fragile or missing enterprise features.",
        customer_next_trajectory: "They will use the competitor's feature checklist to make your pricing seem unjustified.",
        framework_name: "Implication Questioning & Total Cost of Ownership",
        author_and_book: "Neil Rackham — 'SPIN Selling'",
        psychological_principle: "Shifts conversation from upfront purchase price to the catastrophic cost of secondary failures and downtime.",
        exact_script: "When you look at their lower price, what specific architecture shortcuts or compliance compromises are they taking to hit that tier?",
        secondary_followup: "If their system drops out during peak traffic or fails an audit, what does that 2-hour downtime cost your business?",
        tone_delivery_guide: "Objective consultant posture. Never disparage the competitor; ask questions that expose their gaps.",
    },

    // 4. THE SEND ME AN EMAIL / STALL
    Battlecard {
        id: "klaff_send_email_brush",
        category: ObjectionCategory::TimingAndStalling,
        trigger_patterns: &[
            "send me an email",
            "just email me the info",
            "send me a deck",
            "put it in writing",
            "email me your pricing",
        ],
        customer_unspoken_thought: "I want to get off this call without saying 'no' directly. Once you email me, I can safely ignore your follow-ups forever.",
        customer_next_trajectory: "They will ghost your follow-up emails and leave you stuck in the pipeline graveyard.",
        framework_name: "Time Frame Control & Status Inversion",
        author_and_book: "Oren Klaff — 'Pitch Anything'",
        psychological_principle: "Refuses the subservient 'vendor' posture and forces an immediate qualification moment.",
        exact_script: "I can definitely send a deck, but usually when people say 'send me an email,' it's a polite way of saying this isn't a priority right now. Is that what's happening?",
        secondary_followup: "If it's not a fit, I'd rather give you your afternoon back right now than spam your inbox for three weeks.",
        tone_delivery_guide: "Relaxed, detached, zero desperation. Willing to walk away.",
    },

    // 5. I NEED TO TALK TO MY BOSS / CEO
    Battlecard {
        id: "voss_authority_dodge",
        category: ObjectionCategory::AuthorityAndCommitment,
        trigger_patterns: &[
            "need to talk to my boss",
            "check with my team",
            "need executive approval",
            "run this by the committee",
            "speak with the ceo",
            "not my sole decision",
        ],
        customer_unspoken_thought: "I don't want to carry the political burden of pitching this internally unless you give me the exact ammunition to look like a hero.",
        customer_next_trajectory: "They will pitch a weak, 2-minute half-baked summary to their boss, get shot down immediately, and tell you 'we decided to hold off.'",
        framework_name: "Accusation Audit & Champion Arming",
        author_and_book: "Chris Voss & Matthew Dixon — 'Never Split the Difference' & 'The Challenger Sale'",
        psychological_principle: "Transforms the prospect from an insecure messenger into an empowered internal champion.",
        exact_script: "When you present this to your executive team, what's the #1 objection they're going to hit you with to shut it down?",
        secondary_followup: "Let's build the 1-page financial proof document together so you don't have to defend technical architecture alone in that meeting.",
        tone_delivery_guide: "Partnership tone. Act as their internal strategist against their company's bureaucracy.",
    },

    // 6. WE'RE HAPPY WITH WHAT WE HAVE / STATUS QUO
    Battlecard {
        id: "challenger_status_quo",
        category: ObjectionCategory::StatusQuoInertia,
        trigger_patterns: &[
            "happy with what we have",
            "current solution is fine",
            "we're good right now",
            "already have a vendor",
            "if it ain't broke",
            "not looking to switch",
        ],
        customer_unspoken_thought: "Switching tools is painful and requires retraining my team. The pain of changing exceeds the pain of my current minor annoyance.",
        customer_next_trajectory: "They will politely end the conversation and maintain their suboptimal existing stack.",
        framework_name: "Commercial Reframe & Constructive Tension",
        author_and_book: "Matthew Dixon & Brent Adamson — 'The Challenger Sale'",
        psychological_principle: "Introduces unconsidered needs and unseen operational debt to shatter false complacency.",
        exact_script: "Most companies we talk to felt their setup was fine too—until they realized their competitors were closing cycles 3x faster using automated gating.",
        secondary_followup: "What happens to your quarterly targets if your current stack continues leaking 15 hours a week in engineering waste?",
        tone_delivery_guide: "Direct, provocative, authoritative. Bring fresh market insight.",
    },

    // 7. WE ALREADY DO THIS IN SPREADSHEETS / IN-HOUSE
    Battlecard {
        id: "spin_inhouse_spreadsheets",
        category: ObjectionCategory::StatusQuoInertia,
        trigger_patterns: &[
            "we do this in spreadsheets",
            "built our own tool",
            "internal system",
            "we can build this ourselves",
            "our engineers can do this",
            "in-house solution",
        ],
        customer_unspoken_thought: "We pride ourselves on technical self-reliance, even though our internal scripts break every sprint and nobody maintains them.",
        customer_next_trajectory: "They will sink 6 months of senior engineering payroll into building a brittle clone instead of focusing on core product.",
        framework_name: "Problem-to-Implication Escalation",
        author_and_book: "Neil Rackham — 'SPIN Selling'",
        psychological_principle: "Quantifies the real opportunity cost of distracting core developers from revenue-generating product features.",
        exact_script: "Your engineers can definitely build this—but do you want your top developers debugging internal workflows or shipping your core product?",
        secondary_followup: "When your internal tool breaks during a critical release cycle, who gets pulled off customer-facing bugs to fix it?",
        tone_delivery_guide: "Respectful acknowledgment of their team's talent, followed by ruthless business prioritization.",
    },

    // 8. WE'RE GOING TO THINK ABOUT IT / DELAY
    Battlecard {
        id: "cialdini_think_about_it",
        category: ObjectionCategory::TimingAndStalling,
        trigger_patterns: &[
            "we need to think about it",
            "give us some time",
            "circle back in a month",
            "revisit next quarter",
            "need time to digest",
        ],
        customer_unspoken_thought: "I have an unspoken objection or fear I haven't articulated to you yet, so I'm using 'time' as an escape hatch.",
        customer_next_trajectory: "Momentum dies. 70% of deals that enter 'think about it' purgatory never close.",
        framework_name: "The Micro-Commitment & Isolation Technique",
        author_and_book: "Robert Cialdini & Chris Voss — 'Influence' & 'Never Split the Difference'",
        psychological_principle: "Forces the hidden obstacle into the open by removing all other variables.",
        exact_script: "Usually when someone says they need to think about it, it comes down to one of two things: either they don't believe the system works, or the price doesn't make sense. Which one is it?",
        secondary_followup: "What specific piece of information could we clarify right now so you don't have to carry this decision around all week?",
        tone_delivery_guide: "Direct and unhurried. Hold eye contact or pause firmly after the question.",
    },

    // 9. NEVER HEARD OF YOU / TOO SMALL / CREDIBILITY RISK
    Battlecard {
        id: "whitman_credibility_risk",
        category: ObjectionCategory::TrustAndRiskAversion,
        trigger_patterns: &[
            "never heard of your company",
            "you seem small",
            "how long have you been around",
            "you're a startup",
            "nobody ever got fired for buying ibm",
            "what if you go out of business",
        ],
        customer_unspoken_thought: "I don't want to get fired if you guys flop. My personal career safety is more important than your cool technology.",
        customer_next_trajectory: "They will default to an overpriced legacy incumbent solely because it protects their job.",
        framework_name: "LF8 Ego & Social Proof Inversion",
        author_and_book: "Drew Eric Whitman — 'CA$HVERTISING'",
        psychological_principle: "Taps into Life-Force 8 desire for superiority and freedom from fear by framing incumbent tools as outdated career risk.",
        exact_script: "The legacy giants were built 15 years ago before sub-25ms decisioning existed. Teams switch to us because the legacy tools are slowing them down.",
        secondary_followup: "We run completely transparent on your infrastructure with zero lock-in and complete data isolation. Would you like to review our compliance architecture?",
        tone_delivery_guide: "Confident pioneer posture. Frame agility and speed as the superior security choice.",
    },

    // 10. WE DON'T HAVE TIME TO IMPLEMENT
    Battlecard {
        id: "voss_no_time_implement",
        category: ObjectionCategory::TimingAndStalling,
        trigger_patterns: &[
            "no time to implement",
            "team is overwhelmed",
            "bandwidth is tight",
            "too busy right now",
            "can't take on new projects",
        ],
        customer_unspoken_thought: "I'm already working 50 hours a week. If this takes 20 hours to configure, I'll drown.",
        customer_next_trajectory: "They will delay until 'things calm down' (which never happens in business).",
        framework_name: "Radical Simplicity & Reversal of Burden",
        author_and_book: "Chris Voss — 'Never Split the Difference'",
        psychological_principle: "Removes perceived effort hurdle by taking full operational responsibility onto your shoulders.",
        exact_script: "It sounds like your plate is completely overflowing, and the last thing you need is another 30-day onboarding project.",
        secondary_followup: "What if our engineers do the complete setup in under 48 hours, and your team only has to spend 15 minutes reviewing the final dashboard?",
        tone_delivery_guide: "Empathetic relief. Act as an unburdening force in their hectic day.",
    },
];

pub struct MatchResult {
    pub battlecard: &'static Battlecard,
    pub match_score: f32,
    #[allow(dead_code)]
    pub matched_pattern: &'static str,
}

impl Battlecard {
    /// Blazing fast sub-millisecond in-memory matching algorithm
    pub fn find_best_match(query: &str) -> Option<MatchResult> {
        let q = query.to_lowercase();
        let mut best_card: Option<&'static Battlecard> = None;
        let mut best_score: f32 = 0.0;
        let mut best_pat: &'static str = "";

        for card in BATTLECARDS {
            for &pat in card.trigger_patterns {
                let pat_lower = pat.to_lowercase();
                if q.contains(&pat_lower) {
                    let score = (pat_lower.len() as f32) / (q.len().max(pat_lower.len()) as f32) + 0.65;
                    if score > best_score {
                        best_score = score.min(1.0);
                        best_card = Some(card);
                        best_pat = pat;
                    }
                }
            }
        }

        // Secondary semantic keyword fallback if no exact phrase matched
        if best_card.is_none() {
            for card in BATTLECARDS {
                let mut token_hits = 0;
                let mut total_tokens = 0;
                for &pat in card.trigger_patterns {
                    for word in pat.split_whitespace() {
                        total_tokens += 1;
                        if word.len() > 3 && q.contains(word) {
                            token_hits += 1;
                        }
                    }
                }
                if total_tokens > 0 {
                    let ratio = (token_hits as f32) / (total_tokens as f32);
                    if ratio > 0.15 && ratio > best_score {
                        best_score = (ratio + 0.35).min(0.92);
                        best_card = Some(card);
                        best_pat = card.trigger_patterns[0];
                    }
                }
            }
        }

        best_card.map(|b| MatchResult {
            battlecard: b,
            match_score: (best_score * 100.0).round(),
            matched_pattern: best_pat,
        })
    }
}
