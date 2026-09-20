use jev_sales_radar::rag::{Battlecard, ObjectionCategory};

#[test]
fn test_price_objection_matches_chris_voss() {
    let result = Battlecard::find_best_match("That is way too expensive for our company right now").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::PriceAndBudget);
    assert!(result.battlecard.author_and_book.contains("Chris Voss"));
    assert!(result.battlecard.exact_script.contains("raw expense"));
    assert!(result.match_score >= 70.0);
}

#[test]
fn test_price_variation_token_coverage() {
    let result = Battlecard::find_best_match("Your price is way too high compared to our other quotes").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::PriceAndBudget);
    assert!(result.battlecard.author_and_book.contains("Chris Voss"));
}

#[test]
fn test_guarantee_matches_alex_hormozi() {
    let result = Battlecard::find_best_match("What is your money back guarantee if it doesn't work?").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::GuaranteesAndRisk);
    assert!(result.battlecard.author_and_book.contains("Alex Hormozi"));
    assert!(result.battlecard.exact_script.contains("ironclad performance guarantee"));
}

#[test]
fn test_reviews_social_proof_matches_cialdini() {
    let result = Battlecard::find_best_match("Can you show me reviews and case studies of someone in my situation?").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::ReviewsAndSocialProof);
    assert!(result.battlecard.author_and_book.contains("Robert Cialdini"));
    assert!(result.battlecard.exact_script.contains("one of our clients was in your exact situation"));
}

#[test]
fn test_send_email_matches_oren_klaff() {
    let result = Battlecard::find_best_match("Just send me an email with the proposal deck").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::TimingAndStalling);
    assert!(result.battlecard.author_and_book.contains("Oren Klaff"));
    assert!(result.battlecard.exact_script.contains("inbox is probably overflowing"));
}

#[test]
fn test_talk_to_partner_or_boss() {
    let result = Battlecard::find_best_match("I need to talk to my partner and check with my boss before making a move").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::AuthorityAndPartner);
    assert!(result.battlecard.exact_script.contains("what is the #1 concern"));
}

#[test]
fn test_too_good_to_be_true_skepticism() {
    let result = Battlecard::find_best_match("This sounds too good to be true, what is the catch?").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::TrustAndSkepticism);
    assert!(result.battlecard.author_and_book.contains("Drew Eric Whitman"));
    assert!(result.battlecard.exact_script.contains("honest catch"));
}

#[test]
fn test_sub_millisecond_matching_speed() {
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = Battlecard::find_best_match("can you give us a discount");
    }
    let elapsed = start.elapsed();
    let per_query_micros = elapsed.as_micros() / 1000;
    println!("1,000 queries executed in {:?} ({} µs per query)", elapsed, per_query_micros);
    assert!(per_query_micros < 150, "Micro-RAG matching must be under 150 microseconds in debug mode!");
}
