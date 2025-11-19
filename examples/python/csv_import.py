"""
RAG Enricher - CSV インポート/エクスポート サンプル

CSVファイルからナレッジをインポートし、
検索・エクスポート機能をデモンストレーションします。
"""

import csv
from rag_enricher import PyRAGEnricher as RAGEnricher


def create_sample_csv(filename: str):
    """サンプルCSVファイルを作成"""
    print(f"📝 サンプルCSV作成中: {filename}")

    data = [
        {
            "question": "永代供養とは何ですか？",
            "answer": "永代供養とは、お墓の管理を寺院に委託する供養形態です。継承者がいない方でも安心して利用できます。",
            "category": "永代供養",
            "priority": "high"
        },
        {
            "question": "永代供養の費用は？",
            "answer": "永代供養の費用は、一般的に10万円〜150万円程度です。個別安置期間の長さにより価格が変動します。",
            "category": "料金",
            "priority": "high"
        },
        {
            "question": "納骨堂の種類は？",
            "answer": "納骨堂には、ロッカー式、仏壇式、自動搬送式などのタイプがあります。都市部で人気が高まっています。",
            "category": "納骨堂",
            "priority": "medium"
        },
        {
            "question": "樹木葬とは？",
            "answer": "樹木葬は、墓石の代わりに樹木を墓標とする自然葬の一種です。環境に優しく、費用も比較的安価です。",
            "category": "樹木葬",
            "priority": "medium"
        },
        {
            "question": "一般墓との違いは？",
            "answer": "一般墓は家族代々で受け継がれますが、永代供養墓は寺院が永続的に管理します。継承者不要が大きな違いです。",
            "category": "永代供養",
            "priority": "medium"
        },
    ]

    with open(filename, 'w', newline='', encoding='utf-8') as f:
        writer = csv.DictWriter(f, fieldnames=["question", "answer", "category", "priority"])
        writer.writeheader()
        writer.writerows(data)

    print(f"✅ {len(data)} 件のデータを含むCSVを作成\n")


def import_csv_demo(rag: RAGEnricher):
    """CSVインポートのデモ"""
    print("=" * 60)
    print("📥 CSV インポート デモ")
    print("=" * 60)
    print()

    # サンプルCSVを作成
    csv_file = "./faq_data.csv"
    create_sample_csv(csv_file)

    # コレクション作成
    try:
        rag.create_collection("faq", "FAQデータ")
        print("✅ コレクション作成完了\n")
    except:
        print("⚠️  コレクションは既に存在します\n")

    # CSVインポート
    print(f"📥 CSVをインポート中: {csv_file}")

    try:
        count = rag.import_csv(
            file_path=csv_file,
            collection="faq",
            content_column="answer",  # この列をドキュメントとして使用
            metadata_columns=["category", "priority"]  # メタデータとして保存
        )
        print(f"✅ {count} 件のドキュメントをインポート完了\n")
    except Exception as e:
        print(f"❌ インポートエラー: {e}\n")
        return

    # コレクション情報を確認
    collection = rag.get_collection("faq")
    print("📊 コレクション情報:")
    print(f"  名前: {collection.name}")
    print(f"  ドキュメント数: {collection.document_count}")
    print()


def search_demo(rag: RAGEnricher):
    """検索のデモ"""
    print("=" * 60)
    print("🔍 検索デモ")
    print("=" * 60)
    print()

    queries = [
        "永代供養の料金について知りたい",
        "樹木葬とは何ですか",
        "継承者がいない場合の選択肢は？"
    ]

    for i, query in enumerate(queries, 1):
        print(f"\n[質問 {i}] {query}")
        print("-" * 60)

        results = rag.search(
            query=query,
            collection="faq",
            top_k=2,
            threshold=0.0
        )

        for j, result in enumerate(results, 1):
            print(f"\n  結果 {j} (スコア: {result.score:.3f})")
            print(f"  {result.content}")

        print()


def export_csv_demo(rag: RAGEnricher):
    """CSVエクスポートのデモ"""
    print("=" * 60)
    print("📤 CSV エクスポート デモ")
    print("=" * 60)
    print()

    export_file = "./faq_export.csv"

    print(f"📤 データをエクスポート中: {export_file}")

    try:
        count = rag.export_csv(
            file_path=export_file,
            collection="faq"
        )
        print(f"✅ {count} 件のドキュメントをエクスポート完了\n")

        # エクスポートしたCSVの内容を確認
        print("📄 エクスポートされたCSVの内容:")
        with open(export_file, 'r', encoding='utf-8') as f:
            lines = f.readlines()[:6]  # 最初の6行のみ表示
            for line in lines:
                print(f"  {line.strip()}")

        print()

    except Exception as e:
        print(f"❌ エクスポートエラー: {e}\n")


def enrich_demo(rag: RAGEnricher):
    """エンリッチのデモ"""
    print("=" * 60)
    print("✨ エンリッチ デモ (LLM へのコンテキスト生成)")
    print("=" * 60)
    print()

    question = "永代供養と樹木葬の違いを教えてください"
    print(f"💭 質問: {question}\n")

    result = rag.enrich(
        query=question,
        collection="faq",
        top_k=3,
        threshold=0.0
    )

    print("📋 生成されたコンテキスト:")
    print("=" * 60)
    print(result.context)
    print("=" * 60)
    print()

    print("💡 このコンテキストをLLMのプロンプトに含めることで、")
    print("   正確な情報に基づいた回答を生成できます。")
    print()


def main():
    print("🚀 RAG Enricher - CSV インポート/エクスポート デモ")
    print()

    # 初期化
    rag = RAGEnricher(
        db_path="./csv_demo.db",
        model="bge-small-en-v1.5"
    )

    # 1. CSVインポート
    import_csv_demo(rag)

    # 2. 検索
    search_demo(rag)

    # 3. エンリッチ
    enrich_demo(rag)

    # 4. CSVエクスポート
    export_csv_demo(rag)

    print("🎉 デモ完了！")
    print()
    print("📁 生成されたファイル:")
    print("  - csv_demo.db      (SQLiteデータベース)")
    print("  - faq_data.csv     (インポート元CSV)")
    print("  - faq_export.csv   (エクスポートされたCSV)")
    print()


if __name__ == "__main__":
    main()
