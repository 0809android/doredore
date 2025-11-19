#!/usr/bin/env python3
"""
RAG Enricher - 簡単な動作テスト

このスクリプトは、ビルドされたライブラリが正しく動作するかテストします。
"""

print("=" * 60)
print("RAG Enricher - 動作テスト")
print("=" * 60)
print()

# Step 1: インポートテスト
print("1. モジュールのインポート...")
try:
    from rag_enricher import PyRAGEnricher as RAGEnricher
    print("   ✅ インポート成功！")
except ImportError as e:
    print(f"   ❌ インポート失敗: {e}")
    print()
    print("注意: wheelファイルをインストールする必要があります:")
    print("  pip3 install target/wheels/rag_enricher-*.whl")
    exit(1)

print()

# Step 2: 初期化テスト
print("2. RAGEnricher の初期化...")
try:
    rag = RAGEnricher(
        db_path="./test_knowledge.db",
        model="bge-small-en-v1.5",
        cache_dir=None
    )
    print("   ✅ 初期化成功！")
except Exception as e:
    print(f"   ❌ 初期化失敗: {e}")
    exit(1)

print()

# Step 3: コレクション作成テスト
print("3. コレクション作成...")
try:
    collection_id = rag.create_collection("test", "テストコレクション")
    print(f"   ✅ コレクション作成成功！(ID: {collection_id})")
except Exception as e:
    print(f"   ⚠️  コレクション作成: {e} (既に存在する可能性)")

print()

# Step 4: ドキュメント追加テスト
print("4. ドキュメント追加...")
try:
    doc_id = rag.add_document(
        content="これはテストドキュメントです。RAG Enricherの動作確認用です。",
        collection="test"
    )
    print(f"   ✅ ドキュメント追加成功！(ID: {doc_id})")
except Exception as e:
    print(f"   ❌ ドキュメント追加失敗: {e}")
    exit(1)

print()

# Step 5: 検索テスト
print("5. 検索テスト...")
try:
    results = rag.search(
        query="テスト",
        collection="test",
        top_k=3
    )
    print(f"   ✅ 検索成功！({len(results)} 件の結果)")

    if results:
        for i, result in enumerate(results, 1):
            print(f"\n   結果 {i}:")
            print(f"     スコア: {result.score:.3f}")
            print(f"     内容: {result.content[:50]}...")
except Exception as e:
    print(f"   ❌ 検索失敗: {e}")
    exit(1)

print()

# Step 6: エンリッチテスト
print("6. エンリッチ（RAGメイン機能）テスト...")
try:
    enrich_result = rag.enrich(
        query="RAG Enricherについて教えて",
        collection="test",
        top_k=1
    )
    print("   ✅ エンリッチ成功！")
    print()
    print("   生成されたコンテキスト:")
    print("   " + "-" * 56)
    print("   " + enrich_result.context[:100] + "...")
    print("   " + "-" * 56)
except Exception as e:
    print(f"   ❌ エンリッチ失敗: {e}")
    exit(1)

print()
print("=" * 60)
print("🎉 全てのテストが成功しました！")
print("=" * 60)
print()
print("次のステップ:")
print("  - examples/python/basic.py を実行")
print("  - examples/python/with_openai.py を実行（OpenAI API Key必要）")
print("  - examples/python/csv_import.py を実行")
