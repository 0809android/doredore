use serde::{Deserialize, Serialize};

/// 検索モード
/// RAGシステムで使用可能な3種類の検索アルゴリズムを定義
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SearchMode {
    /// セマンティック検索（意味ベース）
    /// - BGE/E5などの埋め込みモデルでベクトル化
    /// - コサイン類似度で意味的な類似性を計算
    /// - 言い換えや類義語にも対応可能
    Semantic,

    /// キーワード検索（完全一致ベース）
    /// - 英語: SQLite FTS5 + BM25アルゴリズム
    /// - 日本語: LIKE検索（FTS5で結果がない場合の自動フォールバック）
    /// - 正確なキーワードマッチングに最適
    Keyword,

    /// ハイブリッド検索（セマンティック + キーワード）
    /// - 両方の検索結果を加重平均で統合
    /// - デフォルト重み: セマンティック 0.7、キーワード 0.3
    /// - 意味理解と正確性のバランスを取る
    Hybrid,
}

impl Default for SearchMode {
    /// デフォルトはセマンティック検索
    /// 多くのRAGユースケースで最も汎用性が高い
    fn default() -> Self {
        SearchMode::Semantic
    }
}

/// 検索結果の単一アイテム
/// 各ドキュメントの検索スコアとメタデータを含む
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// ドキュメントID（データベースの主キー）
    pub document_id: i64,

    /// ドキュメントの本文テキスト
    pub content: String,

    /// 類似度スコア（0.0〜1.0、高いほど関連性が高い）
    /// - セマンティック検索: コサイン類似度
    /// - キーワード検索: 正規化されたBM25スコア
    /// - ハイブリッド: 加重平均スコア
    pub score: f32,

    /// ドキュメントに関連付けられたメタデータ（JSON形式）
    pub metadata: Option<serde_json::Value>,

    /// このドキュメントが属するコレクション名
    pub collection_name: String,
}

/// RAGエンリッチメント結果
/// ユーザーの質問に対する検索結果とコンテキストを格納
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichResult {
    /// 元の質問文
    pub question: String,

    /// LLMに渡すための整形済みコンテキスト文字列
    /// 各ソースがスコアとコレクション名付きでフォーマット済み
    pub context: String,

    /// 検索で取得されたソースドキュメントのリスト
    pub sources: Vec<SearchResult>,
}

impl SearchResult {
    /// 新しい検索結果を作成
    ///
    /// # 引数
    /// * `document_id` - ドキュメントの一意なID
    /// * `content` - ドキュメントの本文
    /// * `score` - 類似度スコア（0.0〜1.0）
    /// * `metadata` - オプショナルなメタデータ
    /// * `collection_name` - コレクション名
    pub fn new(
        document_id: i64,
        content: String,
        score: f32,
        metadata: Option<serde_json::Value>,
        collection_name: String,
    ) -> Self {
        Self {
            document_id,
            content,
            score,
            metadata,
            collection_name,
        }
    }
}

impl EnrichResult {
    /// 新しいエンリッチメント結果を作成
    ///
    /// ソースドキュメントを整形してLLMに渡しやすい形式のコンテキストを生成
    ///
    /// # 引数
    /// * `question` - ユーザーの質問文
    /// * `sources` - 検索で取得されたドキュメントのリスト
    ///
    /// # コンテキストフォーマット
    /// ```text
    /// [Source 1] (Score: 0.876, Collection: docs)
    /// ドキュメントの内容...
    ///
    /// [Source 2] (Score: 0.754, Collection: docs)
    /// ドキュメントの内容...
    /// ```
    pub fn new(question: String, sources: Vec<SearchResult>) -> Self {
        // 各ソースをLLM向けに整形
        let context = sources
            .iter()
            .enumerate()
            .map(|(i, result)| {
                format!(
                    "[Source {}] (Score: {:.3}, Collection: {})\n{}",
                    i + 1,
                    result.score,
                    result.collection_name,
                    result.content
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"); // ソース間を空行で区切る

        Self {
            question,
            context,
            sources,
        }
    }
}

/// コサイン類似度の計算
///
/// 2つのベクトル間の角度に基づいた類似度を計算（-1.0〜1.0）
/// 実際には0.0〜1.0の範囲に収まることが多い
///
/// # 数式
/// ```text
/// cosine_similarity(A, B) = (A · B) / (||A|| × ||B||)
/// ```
///
/// # 引数
/// * `a` - 第1のベクトル（通常はクエリのembedding）
/// * `b` - 第2のベクトル（通常はドキュメントのembedding）
///
/// # 戻り値
/// * 類似度スコア（-1.0〜1.0）
///   - 1.0: 完全に同じ方向（最も類似）
///   - 0.0: 直交（無関係）
///   - -1.0: 正反対（対照的）
/// * ベクトル長が異なる場合やゼロベクトルの場合は0.0を返す
///
/// # 計算量
/// O(d) ここでdはベクトルの次元数（BGE-small-en-v1.5では384次元）
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    // ベクトルの次元数が一致しない場合は類似度なしと判定
    if a.len() != b.len() {
        return 0.0;
    }

    // 内積を計算 (A · B)
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

    // 各ベクトルのノルム（長さ）を計算
    // ||A|| = sqrt(a1² + a2² + ... + an²)
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    // ゼロベクトルの場合は類似度を計算できない
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    // コサイン類似度 = 内積 / (ノルムの積)
    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let vec = vec![1.0, 2.0, 3.0];
        let similarity = cosine_similarity(&vec, &vec);
        assert!((similarity - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert!(similarity.abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![-1.0, 0.0, 0.0];
        let similarity = cosine_similarity(&a, &b);
        assert!((similarity + 1.0).abs() < 1e-6);
    }
}
