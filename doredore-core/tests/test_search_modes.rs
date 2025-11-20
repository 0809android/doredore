use doredore_core::{Doredore, SearchMode};
use std::fs;

#[test]
fn test_three_search_modes() {
    // テスト用DBファイル
    let db_path = "./test_search_modes.db";

    // 既存のDBがあれば削除
    if fs::metadata(db_path).is_ok() {
        fs::remove_file(db_path).unwrap();
    }

    // Doredore初期化
    let rag = Doredore::new(db_path, Some("bge-small-en-v1.5"), None).unwrap();

    // コレクション作成
    rag.create_collection("test", Some("Test collection")).unwrap();

    // テストドキュメント追加
    let docs = vec![
        "永代供養とは、お墓の管理や供養を寺院が永代にわたって行ってくれる供養形態です。",
        "永代供養の費用は、一般的に10万円から150万円程度です。",
        "樹木葬は、墓石の代わりに樹木を墓標とする埋葬方法です。費用は20万円から80万円程度です。",
    ];

    for doc in docs {
        rag.add_document(doc, "test", None).unwrap();
    }

    let query = "永代供養の費用";

    // 1. Semantic Search
    println!("\n=== Semantic Search ===");
    let results = rag.search(
        query,
        Some("test"),
        None,
        3,
        0.0,
        SearchMode::Semantic,
        None,
    ).unwrap();

    for (i, result) in results.iter().enumerate() {
        let preview = result.content.chars().take(30).collect::<String>();
        println!(
            "[{}] Score: {:.4} - {}...",
            i + 1,
            result.score,
            preview
        );
    }
    assert!(!results.is_empty());

    // 2. Keyword Search（日本語LIKE検索）
    println!("\n=== Keyword Search (Japanese) ===");
    let results = rag.search(
        "費用",  // キーワード検索
        Some("test"),
        None,
        3,
        0.0,
        SearchMode::Keyword,
        None,
    ).unwrap();

    for (i, result) in results.iter().enumerate() {
        let preview = result.content.chars().take(30).collect::<String>();
        println!(
            "[{}] Score: {:.4} - {}...",
            i + 1,
            result.score,
            preview
        );
    }
    assert!(!results.is_empty(), "日本語キーワード検索は結果を返すべき");

    // 3. Hybrid Search
    println!("\n=== Hybrid Search (0.7, 0.3) ===");
    let results = rag.search(
        query,
        Some("test"),
        None,
        3,
        0.0,
        SearchMode::Hybrid,
        Some((0.7, 0.3)),  // semantic重視
    ).unwrap();

    for (i, result) in results.iter().enumerate() {
        let preview = result.content.chars().take(30).collect::<String>();
        println!(
            "[{}] Score: {:.4} - {}...",
            i + 1,
            result.score,
            preview
        );
    }
    assert!(!results.is_empty());

    // クリーンアップ
    fs::remove_file(db_path).unwrap();

    println!("\n✅ All three search modes work correctly!");
}
