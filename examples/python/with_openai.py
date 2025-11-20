"""
doredore + OpenAI 統合サンプル

doredore で取得したコンテキストを使って、
OpenAI の GPT モデルで回答を生成します。
"""

import os
from doredore import PyDoredore as Doredore

# OpenAI のインストールが必要: pip install openai
try:
    from openai import OpenAI
except ImportError:
    print("❌ OpenAI ライブラリがインストールされていません")
    print("   pip install openai を実行してください")
    exit(1)


def setup_knowledge_base(rag: Doredore):
    """ナレッジベースをセットアップ"""
    print("📚 ナレッジベースをセットアップ中...\n")

    # コレクション作成
    try:
        rag.create_collection("faq", "よくある質問")
    except:
        pass  # 既に存在する場合

    # サンプルデータ
    knowledge = [
        {
            "content": "永代供養とは、お墓の管理を寺院に委託する供養形態です。継承者がいない方でも安心して利用できます。",
            "metadata": {"category": "永代供養", "importance": "high"}
        },
        {
            "content": "永代供養の費用は、一般的に10万円〜150万円程度です。個別安置期間の長さにより価格が変動します。",
            "metadata": {"category": "料金", "importance": "high"}
        },
        {
            "content": "納骨堂には、ロッカー式、仏壇式、自動搬送式などのタイプがあります。都市部で人気が高まっています。",
            "metadata": {"category": "納骨堂", "importance": "medium"}
        },
        {
            "content": "樹木葬は、墓石の代わりに樹木を墓標とする自然葬の一種です。環境に優しく、費用も比較的安価です。",
            "metadata": {"category": "樹木葬", "importance": "medium"}
        },
    ]

    for item in knowledge:
        try:
            rag.add_document(
                content=item["content"],
                collection="faq",
                metadata=None  # metadata は後で実装
            )
        except Exception as e:
            print(f"⚠️  ドキュメント追加エラー: {e}")

    print("✅ ナレッジベースのセットアップ完了\n")


def chat_with_rag(rag: Doredore, openai_client: OpenAI, question: str):
    """RAG + OpenAI でチャット"""

    print(f"💭 質問: {question}\n")

    # 1. RAG でコンテキストを取得
    print("🔍 関連情報を検索中...")
    enrich_result = rag.enrich(
        query=question,
        collection="faq",
        top_k=3,
        threshold=0.0
    )

    print(f"✅ {len(enrich_result.sources)} 件の関連情報を取得\n")

    # 検索結果を表示
    print("📊 検索結果:")
    for i, source in enumerate(enrich_result.sources, 1):
        print(f"  [{i}] スコア: {source.score:.3f}")
        print(f"      {source.content}\n")

    # 2. OpenAI で回答生成
    print("🤖 AI が回答を生成中...\n")

    try:
        response = openai_client.chat.completions.create(
            model="gpt-4o-mini",
            messages=[
                {
                    "role": "system",
                    "content": f"""あなたは親切な供養コンサルタントです。
以下の情報を参考に、ユーザーの質問に丁寧に回答してください。

【参考情報】
{enrich_result.context}

回答は簡潔で分かりやすく、専門用語は説明を加えてください。"""
                },
                {
                    "role": "user",
                    "content": question
                }
            ],
            temperature=0.7,
            max_tokens=500
        )

        answer = response.choices[0].message.content

        print("=" * 60)
        print("💡 回答:")
        print("=" * 60)
        print(answer)
        print("=" * 60)
        print()

        # ソース情報を表示
        print("📚 参考にした情報:")
        for i, source in enumerate(enrich_result.sources, 1):
            print(f"  [{i}] {source.content[:60]}... (スコア: {source.score:.3f})")

        print()

    except Exception as e:
        print(f"❌ OpenAI API エラー: {e}")


def main():
    # 環境変数チェック
    if "OPENAI_API_KEY" not in os.environ:
        print("❌ OPENAI_API_KEY 環境変数が設定されていません")
        print("   export OPENAI_API_KEY='your-api-key' を実行してください")
        return

    # 初期化
    print("🚀 doredore を初期化中...\n")
    rag = Doredore(
        db_path="./knowledge_with_ai.db",
        model="bge-small-en-v1.5"
    )

    openai_client = OpenAI(api_key=os.environ["OPENAI_API_KEY"])

    # ナレッジベースのセットアップ
    setup_knowledge_base(rag)

    # サンプル質問
    questions = [
        "永代供養の費用はどれくらいですか？",
        "樹木葬と納骨堂の違いを教えてください",
        "継承者がいない場合、どうすればいいですか？"
    ]

    for i, question in enumerate(questions, 1):
        print(f"\n{'='*60}")
        print(f"質問 {i}/{len(questions)}")
        print(f"{'='*60}\n")
        chat_with_rag(rag, openai_client, question)

    print("\n🎉 デモ完了！")


if __name__ == "__main__":
    main()
