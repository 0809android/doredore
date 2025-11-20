"""
doredore - 基本的な使い方

このサンプルでは以下を実演します:
- Doredore の初期化
- コレクションの作成
- ドキュメントの追加
- 検索とエンリッチ
"""

from doredore import PyDoredore as Doredore

def main():
    # 1. Doredore を初期化
    print("🚀 doredore を初期化中...")
    rag = Doredore(
        db_path="./knowledge.db",
        model="bge-small-en-v1.5",  # 軽量モデル (384次元)
        cache_dir=None  # デフォルトキャッシュディレクトリを使用
    )
    print("✅ 初期化完了\n")

    # 2. コレクションを作成
    print("📁 コレクションを作成中...")
    try:
        collection_id = rag.create_collection(
            name="faq",
            description="よくある質問"
        )
        print(f"✅ コレクション作成完了 (ID: {collection_id})\n")
    except Exception as e:
        print(f"⚠️  コレクションは既に存在します: {e}\n")

    # 3. ドキュメントを追加
    print("📝 ドキュメントを追加中...")

    documents = [
        "永代供養とは、お墓の管理を寺院に委託する供養形態です。",
        "納骨堂には、ロッカー式、仏壇式、自動搬送式などがあります。",
        "樹木葬は、墓石の代わりに樹木を墓標とする自然葬の一種です。",
        "一般墓は、家族代々で受け継がれる従来型のお墓です。",
        "永代供養墓は、継承者がいない方でも安心して利用できます。"
    ]

    for i, doc in enumerate(documents, 1):
        try:
            doc_id = rag.add_document(
                content=doc,
                collection="faq",
                metadata=None
            )
            print(f"  ✓ ドキュメント {i} 追加 (ID: {doc_id})")
        except Exception as e:
            print(f"  ✗ エラー: {e}")

    print()

    # 4. コレクション情報を確認
    print("📊 コレクション情報:")
    collection = rag.get_collection("faq")
    print(f"  名前: {collection.name}")
    print(f"  説明: {collection.description}")
    print(f"  ドキュメント数: {collection.document_count}")
    print()

    # 5. 検索を実行
    print("🔍 検索を実行中...")
    query = "永代供養について教えて"
    print(f"  質問: {query}\n")

    results = rag.search(
        query=query,
        collection="faq",
        top_k=3,
        threshold=0.0
    )

    print(f"  検索結果 ({len(results)} 件):")
    for i, result in enumerate(results, 1):
        print(f"\n  [{i}] スコア: {result.score:.3f}")
        print(f"      {result.content}")

    print()

    # 6. エンリッチ (RAGのメイン機能)
    print("✨ エンリッチを実行中...")
    enrich_result = rag.enrich(
        query=query,
        collection="faq",
        top_k=3,
        threshold=0.0
    )

    print(f"\n📋 生成されたコンテキスト:")
    print("-" * 60)
    print(enrich_result.context)
    print("-" * 60)
    print()

    # 7. ドキュメント一覧を取得
    print("📄 全ドキュメント一覧:")
    docs = rag.list_documents(collection="faq", limit=100, offset=0)
    for i, doc in enumerate(docs, 1):
        print(f"  {i}. {doc.content[:50]}...")

    print()
    print("🎉 完了！")

if __name__ == "__main__":
    main()
