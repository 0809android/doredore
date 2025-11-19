use rag_enricher_core::{RAGEnricher, SearchMode};
use std::fs;

#[test]
fn test_all_search_modes_english() {
    let db_path = "./test_search_modes_en.db";

    if fs::metadata(db_path).is_ok() {
        fs::remove_file(db_path).unwrap();
    }

    let rag = RAGEnricher::new(db_path, Some("bge-small-en-v1.5"), None).unwrap();
    rag.create_collection("docs", Some("Documents")).unwrap();

    // 英語ドキュメント
    let docs = vec![
        "Machine learning is a subset of artificial intelligence that focuses on algorithms.",
        "The cost of machine learning projects ranges from 10k to 150k dollars.",
        "Deep learning uses neural networks with multiple layers for complex pattern recognition.",
    ];

    for doc in docs {
        rag.add_document(doc, "docs", None).unwrap();
    }

    // 1. Semantic Search
    println!("\n=== Semantic Search ===");
    let results = rag.search(
        "machine learning cost",
        Some("docs"),
        None,
        3,
        0.0,
        SearchMode::Semantic,
        None,
    ).unwrap();

    for (i, result) in results.iter().enumerate() {
        println!("[{}] Score: {:.4} - {}", i + 1, result.score, result.content);
    }
    assert!(!results.is_empty());

    // 2. Keyword Search
    println!("\n=== Keyword Search ===");
    let results = rag.search(
        "cost",
        Some("docs"),
        None,
        3,
        0.0,
        SearchMode::Keyword,
        None,
    ).unwrap();

    for (i, result) in results.iter().enumerate() {
        println!("[{}] Score: {:.4} - {}", i + 1, result.score, result.content);
    }
    assert!(!results.is_empty(), "Keyword search should return results");

    // 3. Hybrid Search
    println!("\n=== Hybrid Search (0.7, 0.3) ===");
    let results = rag.search(
        "machine learning cost",
        Some("docs"),
        None,
        3,
        0.0,
        SearchMode::Hybrid,
        Some((0.7, 0.3)),
    ).unwrap();

    for (i, result) in results.iter().enumerate() {
        println!("[{}] Score: {:.4} - {}", i + 1, result.score, result.content);
    }
    assert!(!results.is_empty());

    fs::remove_file(db_path).unwrap();

    println!("\n✅ All three search modes work correctly!");
}
