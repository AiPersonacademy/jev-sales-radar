use jev_sales_radar::rag::{Battlecard, ObjectionCategory};

#[test]
fn test_price_objection_matches_chris_voss() {
    let result = Battlecard::find_best_match("That is way too expensive for our company right now").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::PriceAndBudget);
    assert!(result.battlecard.author_and_book.contains("Chris Voss"));
    assert!(result.battlecard.exact_script.contains("pressure from finance"));
    assert!(result.match_score >= 70.0);
}

#[test]
fn test_price_variation_token_coverage() {
    let result = Battlecard::find_best_match("Your price is way too high compared to our other quotes").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::PriceAndBudget);
    assert!(result.battlecard.author_and_book.contains("Chris Voss"));
}

#[test]
fn test_send_email_matches_oren_klaff() {
    let result = Battlecard::find_best_match("Just send me an email with the pricing deck").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::TimingAndStalling);
    assert!(result.battlecard.author_and_book.contains("Oren Klaff"));
    assert!(result.battlecard.exact_script.contains("polite way of saying"));
}

#[test]
fn test_competitor_cheaper_matches_spin_selling() {
    let result = Battlecard::find_best_match("Competitor is cheaper by almost 50%").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::CompetitorComparison);
    assert!(result.battlecard.author_and_book.contains("Neil Rackham"));
    assert!(result.battlecard.exact_script.contains("architecture shortcuts"));
}

#[test]
fn test_talk_to_boss_matches_accusation_audit() {
    let result = Battlecard::find_best_match("I need to talk to my boss before we make any moves").unwrap();
    assert_eq!(result.battlecard.category, ObjectionCategory::AuthorityAndCommitment);
    assert!(result.battlecard.exact_script.contains("executive team"));
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
    assert!(per_query_micros < 50, "Micro-RAG matching must be under 50 microseconds!");
}
