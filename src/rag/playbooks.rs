use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectionCategory {
    PriceAndBudget,
    GuaranteesAndRisk,
    ReviewsAndSocialProof,
    TimingAndStalling,
    AuthorityAndPartner,
    TrustAndSkepticism,
    StatusQuoAndDiy,
    BandwidthAndTime,
    CompetitorComparison,
    GeneralDiscovery,
}

#[derive(Debug, Clone, Serialize)]
pub struct Battlecard {
    pub id: &'static str,
    pub category: ObjectionCategory,
    pub trigger_patterns: &'static [&'static str],
    pub state_of_being: &'static str,
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
    // 1. PRICE TOO HIGH / AFFORDABILITY (General Sales Call)
    Battlecard {
        id: "voss_price_too_high",
        category: ObjectionCategory::PriceAndBudget,
        trigger_patterns: &[
            "too expensive",
            "price is high",
            "price is way too high",
            "over our budget",
            "can't afford",
            "cannot afford",
            "cost too much",
            "sticker shock",
            "out of our price range",
            "that's a lot of money",
            "costs a lot",
        ],
        state_of_being: "Financial Self-Preservation & Fear of Buyer's Remorse",
        customer_unspoken_thought: "I want the promised result, but I'm terrified of making an expensive mistake or overpaying. I need to be 100% sure this pays for itself before risking my capital.",
        customer_next_trajectory: "They are about to say: 'Can you do it for less, or maybe we can start with something smaller?' or 'We'll have to wait until next quarter.'",
        framework_name: "Tactical Empathy & Calibrated ROI Reframe",
        author_and_book: "Chris Voss & Alex Hormozi — 'Never Split the Difference' & '$100M Offers'",
        psychological_principle: "Disarms defensive amygdala activation by vocalizing their fear of waste, then transforms an 'expense' into an inevitable return.",
        exact_script: "It feels like a big number right now because you're looking at this as a raw expense rather than an asset that pays for itself. If you knew with 100% certainty that this completely solved your problem, would the price still hold you back?",
        secondary_followup: "What is it currently costing you every month in lost time and missed results by staying where you are right now?",
        tone_delivery_guide: "Late-night FM DJ voice: calm, slow cadence, downward inflection at sentence end. Completely devoid of defensiveness.",
    },

    // 2. GUARANTEES & REFUNDS (General Sales Call)
    Battlecard {
        id: "hormozi_guarantee_risk",
        category: ObjectionCategory::GuaranteesAndRisk,
        trigger_patterns: &[
            "what's the guarantee",
            "what is the guarantee",
            "is there a guarantee",
            "money back guarantee",
            "what if it doesn't work",
            "what if it does not work",
            "can i get a refund",
            "refund policy",
            "cancel anytime",
            "what if i'm not satisfied",
            "what if i am not satisfied",
        ],
        state_of_being: "Paralyzed Buyer / Risk Aversion / Burned in the Past",
        customer_unspoken_thought: "I've been burned by slick promises before. If this flops, I'm the fool holding the bag. I need the salesperson to shoulder the financial and performance risk, not me.",
        customer_next_trajectory: "They are about to say: 'I need to see your contract and written refund policy before I even consider taking the next step.'",
        framework_name: "Risk Reversal & Conditional Performance Guarantee",
        author_and_book: "Alex Hormozi & Drew Eric Whitman — '$100M Offers' & 'CA$HVERTISING'",
        psychological_principle: "Eliminates perceived decision hazard by transferring 100% of the operational risk from buyer to seller.",
        exact_script: "You shouldn't have to carry the risk for our ability to deliver. We back this with an ironclad performance guarantee: if you follow the system and don't hit the agreed milestones, we work with you for free until you do, or refund every single dollar. Does that take the weight off your shoulders?",
        secondary_followup: "If you have zero financial downside and only upside, what's really keeping you from getting started today?",
        tone_delivery_guide: "Absolute conviction and grounded stability. Unshakable posture that radiates total certainty in the product.",
    },

    // 3. REVIEWS, TESTIMONIALS & SOCIAL PROOF (General Sales Call)
    Battlecard {
        id: "cialdini_reviews_social_proof",
        category: ObjectionCategory::ReviewsAndSocialProof,
        trigger_patterns: &[
            "do you have reviews",
            "show me reviews",
            "show me testimonials",
            "do you have testimonials",
            "case studies",
            "do you have case studies",
            "who else has done this",
            "who else uses this",
            "proof",
            "show me proof",
            "any references",
            "anyone in my situation",
        ],
        state_of_being: "Herd Safety Seeking & Social Validation Need",
        customer_unspoken_thought: "I don't want to be the pioneer or the guinea pig. I need undeniable proof that regular people or companies in my exact shoes took this leap and won.",
        customer_next_trajectory: "They are about to say: 'Can you send me a list of references or case studies with phone numbers that I can contact before deciding?'",
        framework_name: "Third-Party Social Proof & Peer Mirror Story",
        author_and_book: "Robert Cialdini & Oren Klaff — 'Influence' & 'Pitch Anything'",
        psychological_principle: "Humans look to the actions of similar others to guide their own decisions when uncertain.",
        exact_script: "I wouldn't expect you to take my word for it. In fact, one of our clients was in your exact situation two months ago—hesitant, wondering if this would actually work for them. Within 30 days of implementation, they completely turned their numbers around and doubled their output. Would you like me to show you their exact before-and-after breakdown?",
        secondary_followup: "When you see the exact walkthrough of how they did it, what specific metric matters most to your situation?",
        tone_delivery_guide: "Casual storytelling cadence. Warm, authentic, matter-of-fact—like sharing news with a respected colleague.",
    },

    // 4. TIMING & 'LET ME THINK ABOUT IT' (General Sales Call)
    Battlecard {
        id: "voss_think_about_it",
        category: ObjectionCategory::TimingAndStalling,
        trigger_patterns: &[
            "let me think about it",
            "need to think about it",
            "need some time to think",
            "give us some time",
            "i'll sleep on it",
            "circle back next week",
            "circle back next month",
            "revisit next quarter",
            "not ready right now",
            "call me back later",
        ],
        state_of_being: "Polite Avoidance / Fear of Confrontation / Hidden Objection",
        customer_unspoken_thought: "There is an unspoken hesitation or fear I haven't articulated to you, so I'm using 'time' as an easy polite escape hatch so I can hang up and ghost.",
        customer_next_trajectory: "They are about to say: 'Just email me the summary and I'll review it over the weekend and let you know.'",
        framework_name: "The Micro-Commitment Isolation & Truth Probe",
        author_and_book: "Chris Voss & Robert Cialdini — 'Never Split the Difference' & 'Influence'",
        psychological_principle: "Isolates the hidden fear into the open by eliminating false variables and giving permission to speak truth.",
        exact_script: "Usually when someone tells me they need to think about it, it really comes down to one of two things: either they don't believe the mechanism will actually work for them, or they're uncomfortable with the investment. Which one is it for you?",
        secondary_followup: "What specific piece of clarity could we resolve right now so you don't have to carry this decision around on your mind all week?",
        tone_delivery_guide: "Gentle, non-judgmental, spacious. Ask the question and stay completely silent for at least 3 seconds.",
    },

    // 5. SEND ME AN EMAIL / BRUSH OFF (General Sales Call)
    Battlecard {
        id: "klaff_send_email_brush",
        category: ObjectionCategory::TimingAndStalling,
        trigger_patterns: &[
            "send me an email",
            "just email me",
            "send me the info",
            "send me a deck",
            "send me a proposal",
            "send me some information",
            "send over a brochure",
            "put it in an email",
        ],
        state_of_being: "Reflexive Brush-Off / Inbound Overload",
        customer_unspoken_thought: "I'm busy and getting pitched all day. Sending me an email lets me file you in my mental spam folder without feeling guilty.",
        customer_next_trajectory: "They are about to say: 'I'll look over whatever you send and reach back out if it fits.' (95% chance they never reply).",
        framework_name: "The Frame-Flip & Reluctance Disarm",
        author_and_book: "Oren Klaff & Jeb Blount — 'Pitch Anything' & 'Fanatical Prospecting'",
        psychological_principle: "Interrupts the automatic consumer script by refusing the passive inbox brush-off with professional dignity.",
        exact_script: "I can definitely send an email, but candidly, your inbox is probably overflowing and you'll never look at it. What is the single biggest question that, if answered right now, would tell you whether this is a fit or a waste of time?",
        secondary_followup: "If what I send matches everything you're looking for, what would be the very next step on your end?",
        tone_delivery_guide: "Firm, friendly prizing frame. You respect your time and their time equally.",
    },

    // 6. AUTHORITY & PARTNER DODGE (General Sales Call)
    Battlecard {
        id: "voss_partner_spouse_boss",
        category: ObjectionCategory::AuthorityAndPartner,
        trigger_patterns: &[
            "talk to my wife",
            "talk to my husband",
            "check with my partner",
            "check with my business partner",
            "need to talk to my boss",
            "check with my team",
            "need executive approval",
            "not my decision alone",
            "run this by the committee",
        ],
        state_of_being: "Diffused Responsibility / Fear of Internal Conflict",
        customer_unspoken_thought: "I like this, but I don't want to carry the political burden or marital friction of making this decision alone. I'm using them as a shield.",
        customer_next_trajectory: "They are about to say: 'If it were up to me I'd do it right now, but I have to see what they say first.' (They will pitch a weak 30-second summary and get shot down).",
        framework_name: "The Champion Arming & Objection Pre-Mortem",
        author_and_book: "Chris Voss & Matthew Dixon — 'Never Split the Difference' & 'The Challenger Sale'",
        psychological_principle: "Arms the prospect to defend the solution internally rather than letting them become a vulnerable messenger.",
        exact_script: "Makes total sense—you definitely want them aligned on this. When you bring this to them tonight, what is the #1 concern or pushback they're going to hit you with?",
        secondary_followup: "Would it make sense for us to hop on a quick 10-minute 3-way call together so you don't have to defend all the technical details by yourself?",
        tone_delivery_guide: "Allied partnership tone. Position yourself as their internal strategist.",
    },

    // 7. SKEPTICISM & 'TOO GOOD TO BE TRUE' (General Sales Call)
    Battlecard {
        id: "whitman_too_good_to_be_true",
        category: ObjectionCategory::TrustAndSkepticism,
        trigger_patterns: &[
            "too good to be true",
            "sounds too good to be true",
            "what's the catch",
            "what is the catch",
            "is this a scam",
            "sounds like hype",
            "why should i trust you",
            "i'm skeptical",
            "i am skeptical",
            "hard to believe",
        ],
        state_of_being: "Reptilian Defense / Hype Alarm Triggered",
        customer_unspoken_thought: "Everyone in your industry overpromises and underdelivers. My BS detector is buzzing. Show me where the trap is.",
        customer_next_trajectory: "They are about to say: 'There has to be hidden fees or catch you aren't telling me about.'",
        framework_name: "The Damaging Admission & Radical Candor",
        author_and_book: "Drew Eric Whitman & Oren Klaff — 'CA$HVERTISING' & 'Pitch Anything'",
        psychological_principle: "Admitting a real flaw or requirement instantly authenticates all subsequent positive claims.",
        exact_script: "You're completely right to be skeptical. If someone promised me these kinds of results without context, my guard would be up too. Here's the honest catch: this system does not work automatically. It requires consistent execution and disciplined implementation. If you're looking for push-button magic, we're the wrong fit. But if you execute the playbook, the math is undeniable.",
        secondary_followup: "Given that reality, are you looking for an easy shortcut, or a proven infrastructure that actually compounds?",
        tone_delivery_guide: "Disarming, grounded authenticity. Zero sales hype. Pure truth-telling posture.",
    },

    // 8. DO IT OURSELVES / IN-HOUSE DIY (General Sales Call)
    Battlecard {
        id: "rackham_spin_diy_inertia",
        category: ObjectionCategory::StatusQuoAndDiy,
        trigger_patterns: &[
            "we can do this ourselves",
            "can do this ourselves",
            "figure it out myself",
            "we can build this",
            "already have an internal system",
            "built our own tool",
            "can find it on youtube",
            "do it in house",
        ],
        state_of_being: "Overconfident DIY Bias / Hidden Cost Blindness",
        customer_unspoken_thought: "I pride myself on self-reliance. I don't want to spend money on something I think I or my team can cobble together for free.",
        customer_next_trajectory: "They are about to say: 'We'll try tackling it ourselves first, and if we hit a wall, we'll reach back out to you.' (They will lose 6 months of trial-and-error).",
        framework_name: "Implication Escalation & Opportunity Cost Quantification",
        author_and_book: "Neil Rackham — 'SPIN Selling'",
        psychological_principle: "Exposes the catastrophic unseen cost of delay, distraction, and trial-and-error payroll.",
        exact_script: "You definitely could build or figure this out yourselves—your team is talented. But what's the real cost of spending the next 6 to 9 months in trial-and-error while your core business waits? What revenue are you giving up while your focus is diverted?",
        secondary_followup: "Would you rather spend the next half-year reinventing the wheel, or plug in a battle-tested solution on day one and focus entirely on growth?",
        tone_delivery_guide: "Respectful deference to their talent, followed by rigorous business reality-check.",
    },

    // 9. NO TIME / BANDWIDTH / OVERWHELMED (General Sales Call)
    Battlecard {
        id: "voss_no_time_bandwidth",
        category: ObjectionCategory::BandwidthAndTime,
        trigger_patterns: &[
            "no time",
            "don't have time",
            "too busy",
            "too busy right now",
            "team is overwhelmed",
            "bandwidth is tight",
            "looks complicated",
            "steep learning curve",
            "too much on my plate",
        ],
        state_of_being: "Cognitive Exhaustion & Friction Avoidance",
        customer_unspoken_thought: "I'm already drowning in daily fire-fighting. If this takes 20 hours to set up or learn, I will collapse under the weight.",
        customer_next_trajectory: "They are about to say: 'Let's reconnect in 3 to 6 months when things calm down.' (Which never happens).",
        framework_name: "Radical Burden Reversal & Done-With-You Lift",
        author_and_book: "Chris Voss & Alex Hormozi — 'Never Split the Difference' & '$100M Offers'",
        psychological_principle: "Removes psychological friction by taking the heavy cognitive burden entirely off their shoulders.",
        exact_script: "It sounds like you're completely underwater right now, and the last thing you need is another 30-day headache to manage. What if 90% of the heavy lifting is handled for you, so this actually frees up hours in your week instead of taking them?",
        secondary_followup: "If you only had to invest 15 minutes to review the finished system, would that fit into your schedule this week?",
        tone_delivery_guide: "Deep, empathetic relief. Speak as an unburdening ally, not an additional taskmaster.",
    },

    // 10. DISCOUNT BARGAINING / PRICE HAGGLING (General Sales Call)
    Battlecard {
        id: "voss_discount_bargaining",
        category: ObjectionCategory::PriceAndBudget,
        trigger_patterns: &[
            "can you give us a discount",
            "can you give me a discount",
            "any discount",
            "what's your best price",
            "what is your best price",
            "lower the price",
            "sharpen your pencil",
            "give me a deal",
            "can you do better",
        ],
        state_of_being: "Opportunistic Negotiation / Testing Seller Posture",
        customer_unspoken_thought: "I'm testing your spine. If you drop your price in 2 seconds without negotiation, I'll know your product was overpriced and you're desperate for the deal.",
        customer_next_trajectory: "They are about to say: 'If you can knock 25% off right now, I'll pull out my credit card today.'",
        framework_name: "The Ackerman Model & Calibrated Tradeoff",
        author_and_book: "Chris Voss — 'Never Split the Difference'",
        psychological_principle: "Preserves pricing integrity and forces the buyer to confront the real tradeoffs of cutting budget.",
        exact_script: "We price based on guaranteeing the full result without cutting corners. How am I supposed to lower the investment without stripping out the exact support and execution your team asked for?",
        secondary_followup: "If we adjust the scope to fit your budget, which specific deliverables are you comfortable removing from the plan?",
        tone_delivery_guide: "Completely calm, unhurried, curious. Never defensive or panicked.",
    },

    // 11. COMPETITOR COMPARISON (General Sales Call)
    Battlecard {
        id: "rackham_competitor_comparison",
        category: ObjectionCategory::CompetitorComparison,
        trigger_patterns: &[
            "competitor is cheaper",
            "other company is cheaper",
            "other vendor",
            "we got a quote from",
            "why are you more expensive than",
            "cheaper alternative",
            "looking at other options",
        ],
        state_of_being: "Bargaining Leverage Seeking / False Equivalence",
        customer_unspoken_thought: "I'm comparing you to a cheaper, lower-quality option to see if I can force you to match their bottom-dollar rate.",
        customer_next_trajectory: "They are about to say: 'Company X is offering basically the same thing for 40% less. Can you match them?'",
        framework_name: "Value-to-Risk Reframe & Non-Parity Positioning",
        author_and_book: "Neil Rackham & Matthew Dixon — 'SPIN Selling' & 'The Challenger Sale'",
        psychological_principle: "Breaks commoditization by highlighting the hidden operational traps and secondary failures of budget alternatives.",
        exact_script: "They're a well-known option for budget shoppers, but when you look closely, what corners are they cutting on execution or reliability to offer that price? What happens when their system fails right when you need it most?",
        secondary_followup: "Do you want the cheapest initial receipt, or the highest return on investment with zero operational risk?",
        tone_delivery_guide: "Objective advisor. Never badmouth competitors—let their structural limitations speak for themselves.",
    },

    // 12. GENERAL DISCOVERY / BROAD INQUIRY (General Sales Call)
    Battlecard {
        id: "voss_general_discovery",
        category: ObjectionCategory::GeneralDiscovery,
        trigger_patterns: &[
            "how does this work",
            "what do you guys do",
            "what do you do",
            "tell me more",
            "give me the pitch",
            "what is this about",
            "explain your service",
        ],
        state_of_being: "Low Emotional Commitment / Diagnostic Phase",
        customer_unspoken_thought: "I'll listen for 60 seconds, but if you launch into a boring generic monologue about your company, I'm going to tune out and look at my phone.",
        customer_next_trajectory: "They are about to say: 'Okay, so what is your pricing?' to quickly dismiss you.",
        framework_name: "The Calibrated Diagnostic Opener",
        author_and_book: "Chris Voss — 'Never Split the Difference'",
        psychological_principle: "Prevents premature pitching by getting the prospect talking about their specific pain in the first 30 seconds.",
        exact_script: "Before I get into the mechanics of how we do it—what was the #1 frustration or goal that made you carve out time to jump on this call today?",
        secondary_followup: "How long has that been an issue, and what have you tried so far to fix it?",
        tone_delivery_guide: "Deeply curious, attentive, listening posture. Let them speak 80% of the conversation.",
    },
];

pub struct MatchResult {
    pub battlecard: &'static Battlecard,
    pub match_score: f32,
    #[allow(dead_code)]
    pub matched_pattern: &'static str,
}

impl Battlecard {
    /// Sub-millisecond in-memory matching algorithm with zero-allocation fast-path
    pub fn find_best_match(query: &str) -> Option<MatchResult> {
        let q = query.to_lowercase();

        let mut best_card: Option<&'static Battlecard> = None;
        let mut best_score: f32 = 0.0;
        let mut best_pat: &'static str = "";

        // Pass 1: Direct phrase match (zero allocation, all trigger patterns are already lowercase)
        for card in BATTLECARDS {
            for &pat in card.trigger_patterns {
                if q.contains(pat) {
                    let score = 0.85 + (pat.len() as f32 / (q.len().max(pat.len()) as f32) * 0.15);
                    if score > best_score {
                        best_score = score.min(0.99);
                        best_card = Some(card);
                        best_pat = pat;
                    }
                }
            }
        }

        if best_card.is_some() {
            return best_card.map(|b| MatchResult {
                battlecard: b,
                match_score: (best_score * 100.0).round(),
                matched_pattern: best_pat,
            });
        }

        let query_words: Vec<&str> = q.split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();

        // Pass 2: Per-pattern token coverage (e.g. 'price is way too high' matches 'price is high')
        for card in BATTLECARDS {
            for &pat in card.trigger_patterns {
                let pat_words: Vec<&str> = pat.split_whitespace()
                    .filter(|w| w.len() > 2 && !["the", "our", "and", "for", "with", "that", "this"].contains(w))
                    .collect();

                if pat_words.is_empty() {
                    continue;
                }

                let matches = pat_words.iter()
                    .filter(|pw| query_words.iter().any(|qw| qw == *pw || qw.starts_with(*pw)))
                    .count();

                let ratio = matches as f32 / pat_words.len() as f32;
                if ratio >= 0.50 && ratio > best_score {
                    best_score = 0.70 + (ratio * 0.25);
                    best_card = Some(card);
                    best_pat = pat;
                }
            }
        }

        // Pass 3: Core keyword stem trigger fallback
        if best_card.is_none() {
            for card in BATTLECARDS {
                for &pat in card.trigger_patterns {
                    for word in pat.split_whitespace() {
                        if word.len() >= 4 && query_words.iter().any(|qw| qw == &word) {
                            let score = 0.75;
                            if score > best_score {
                                best_score = score;
                                best_card = Some(card);
                                best_pat = pat;
                            }
                        }
                    }
                }
            }
        }

        best_card.map(|b| MatchResult {
            battlecard: b,
            match_score: (best_score.min(0.98) * 100.0).round(),
            matched_pattern: best_pat,
        })
    }
}
