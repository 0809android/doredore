# RAG Enricher - 使用例（全言語）

## Python

### 基本的な使い方
```python
from rag_enricher import RAGEnricher
import openai

# 初期化
rag = RAGEnricher("./knowledge.db")

# コレクション作成
rag.create_collection("faq", "よくある質問")

# ドキュメント追加
rag.add_document(
    "永代供養とは、お墓の管理を寺院に委託する供養形態です。",
    collection="faq"
)

# RAG でコンテキスト付加
result = rag.enrich("永代供養について教えて", collection="faq", top_k=3)

# OpenAI に投げる
response = openai.chat.completions.create(
    model="gpt-4",
    messages=[
        {
            "role": "system",
            "content": f"以下の情報を参考に回答してください:\n\n{result.context}"
        },
        {
            "role": "user",
            "content": result.question
        }
    ]
)

print(response.choices[0].message.content)
```

### FastAPI 統合
```python
from fastapi import FastAPI
from rag_enricher import RAGEnricher

app = FastAPI()
rag = RAGEnricher("./knowledge.db")

@app.post("/chat")
async def chat(question: str):
    result = rag.enrich(question, top_k=3)

    # LLM に投げる処理...

    return {
        "answer": "...",
        "sources": result.sources
    }

@app.on_event("startup")
async def startup():
    # 管理画面起動
    rag.start_admin(port=8080)
```

---

## Node.js / Next.js

### 基本的な使い方 (Node.js)
```typescript
import { RAGEnricher } from 'rag-enricher';
import OpenAI from 'openai';

// 初期化
const rag = new RAGEnricher('./knowledge.db');

// コレクション作成
await rag.createCollection('faq', 'よくある質問');

// ドキュメント追加
await rag.addDocument(
  '永代供養とは、お墓の管理を寺院に委託する供養形態です。',
  { collection: 'faq' }
);

// RAG でコンテキスト付加
const result = await rag.enrich('永代供養について教えて', {
  collection: 'faq',
  topK: 3
});

// OpenAI に投げる
const openai = new OpenAI();
const response = await openai.chat.completions.create({
  model: 'gpt-4',
  messages: [
    {
      role: 'system',
      content: `以下の情報を参考に回答してください:\n\n${result.context}`
    },
    {
      role: 'user',
      content: result.question
    }
  ]
});

console.log(response.choices[0].message.content);
```

### Next.js App Router - API Route
```typescript
// app/api/chat/route.ts
import { RAGEnricher } from 'rag-enricher';
import { NextRequest, NextResponse } from 'next/server';

const rag = new RAGEnricher('./knowledge.db');

export async function POST(request: NextRequest) {
  const { question } = await request.json();

  // RAG でコンテキスト付加
  const result = await rag.enrich(question, { topK: 3 });

  // LLM に投げる処理...

  return NextResponse.json({
    answer: '...',
    sources: result.sources
  });
}
```

### Next.js Server Component
```typescript
// app/knowledge/page.tsx
import { RAGEnricher } from 'rag-enricher';

export default async function KnowledgePage() {
  const rag = new RAGEnricher('./knowledge.db');

  // サーバーサイドで検索
  const results = await rag.search('永代供養', { topK: 5 });

  return (
    <div>
      <h1>ナレッジ検索</h1>
      {results.map(result => (
        <div key={result.documentId}>
          <p>{result.content}</p>
          <span>スコア: {result.score}</span>
        </div>
      ))}
    </div>
  );
}
```

### Express.js 統合
```javascript
import express from 'express';
import { RAGEnricher } from 'rag-enricher';

const app = express();
const rag = new RAGEnricher('./knowledge.db');

app.post('/chat', async (req, res) => {
  const { question } = req.body;

  const result = await rag.enrich(question, { topK: 3 });

  // LLM に投げる処理...

  res.json({
    answer: '...',
    sources: result.sources
  });
});

// 管理画面起動
await rag.startAdmin({ port: 8080 });

app.listen(3000);
```

---

## Ruby / Rails

### 基本的な使い方 (Ruby)
```ruby
require 'rag_enricher'

# 初期化
rag = RagEnricher.new('./knowledge.db')

# コレクション作成
rag.create_collection('faq', 'よくある質問')

# ドキュメント追加
rag.add_document(
  '永代供養とは、お墓の管理を寺院に委託する供養形態です。',
  collection: 'faq'
)

# RAG でコンテキスト付加
result = rag.enrich('永代供養について教えて', collection: 'faq', top_k: 3)

# OpenAI に投げる
client = OpenAI::Client.new
response = client.chat(
  parameters: {
    model: 'gpt-4',
    messages: [
      {
        role: 'system',
        content: "以下の情報を参考に回答してください:\n\n#{result.context}"
      },
      {
        role: 'user',
        content: result.question
      }
    ]
  }
)

puts response.dig('choices', 0, 'message', 'content')
```

### Rails Controller
```ruby
# app/controllers/chat_controller.rb
class ChatController < ApplicationController
  def create
    rag = RagEnricher.new('./knowledge.db')

    result = rag.enrich(params[:question], collection: 'faq', top_k: 3)

    # LLM に投げる処理...

    render json: {
      answer: '...',
      sources: result.sources
    }
  end
end
```

### ActiveRecord 統合パターン
```ruby
# app/models/concerns/rag_searchable.rb
module RagSearchable
  extend ActiveSupport::Concern

  class_methods do
    def sync_to_rag
      rag = RagEnricher.new('./knowledge.db')
      rag.create_collection(table_name, "#{name} データ")

      all.find_each do |record|
        rag.add_document(
          record.to_rag_text,
          collection: table_name,
          metadata: { id: record.id }
        )
      end
    end

    def search_rag(query)
      rag = RagEnricher.new('./knowledge.db')
      rag.search(query, collection: table_name, top_k: 10)
    end
  end

  def to_rag_text
    # Override in model
    raise NotImplementedError
  end
end

# app/models/article.rb
class Article < ApplicationRecord
  include RagSearchable

  def to_rag_text
    "#{title}\n#{content}"
  end
end

# 使用例
Article.sync_to_rag
results = Article.search_rag("検索クエリ")
```

### Sidekiq バックグラウンドジョブ
```ruby
# app/workers/rag_sync_worker.rb
class RagSyncWorker
  include Sidekiq::Worker

  def perform(model_name)
    rag = RagEnricher.new('./knowledge.db')
    model = model_name.constantize

    rag.create_collection(model.table_name, "#{model.name} データ")

    model.find_each do |record|
      rag.add_document(
        record.to_rag_text,
        collection: model.table_name,
        metadata: { id: record.id }
      )
    end
  end
end

# 使用例
RagSyncWorker.perform_async('Article')
```

### Rake タスク
```ruby
# lib/tasks/rag.rake
namespace :rag do
  desc "RAG データベースを初期化"
  task init: :environment do
    rag = RagEnricher.new('./knowledge.db')
    rag.create_collection('articles', 'ブログ記事')
    puts "RAG データベースを初期化しました"
  end

  desc "全記事を RAG に同期"
  task sync: :environment do
    Article.sync_to_rag
    puts "#{Article.count} 件の記事を同期しました"
  end

  desc "管理画面を起動"
  task admin: :environment do
    rag = RagEnricher.new('./knowledge.db')
    rag.start_admin(port: 8080)
    puts "管理画面: http://localhost:8080"
    sleep
  end
end
```

---

## 管理画面の起動

### Python
```python
from rag_enricher import RAGEnricher

rag = RAGEnricher("./knowledge.db")
rag.start_admin(port=8080, host="0.0.0.0")
print("管理画面: http://localhost:8080")
```

### Node.js
```typescript
import { RAGEnricher } from 'rag-enricher';

const rag = new RAGEnricher('./knowledge.db');
await rag.startAdmin({ port: 8080, host: '0.0.0.0' });
console.log('管理画面: http://localhost:8080');
```

### Ruby
```ruby
require 'rag_enricher'

rag = RagEnricher.new('./knowledge.db')
rag.start_admin(port: 8080, host: '0.0.0.0')
puts '管理画面: http://localhost:8080'
```

---

## CSV インポート

### Python
```python
rag = RAGEnricher("./knowledge.db")
rag.create_collection("faq", "FAQ")

# CSV インポート
count = rag.import_csv(
    file_path="./faq_data.csv",
    collection="faq",
    content_column="question",
    metadata_columns=["category", "priority"]
)
print(f"{count}件のデータをインポートしました")
```

### Node.js
```typescript
const rag = new RAGEnricher('./knowledge.db');
await rag.createCollection('faq', 'FAQ');

// CSV インポート
const count = await rag.importCsv({
  filePath: './faq_data.csv',
  collection: 'faq',
  contentColumn: 'question',
  metadataColumns: ['category', 'priority']
});
console.log(`${count}件のデータをインポートしました`);
```

### Ruby
```ruby
rag = RagEnricher.new('./knowledge.db')
rag.create_collection('faq', 'FAQ')

# CSV インポート
count = rag.import_csv(
  file_path: './faq_data.csv',
  collection: 'faq',
  content_column: 'question',
  metadata_columns: ['category', 'priority']
)
puts "#{count}件のデータをインポートしました"
```

---

## 複数コレクション検索

### Python
```python
# 特定のコレクションから検索
result = rag.enrich("質問", collection="faq")

# 複数コレクションから検索
result = rag.enrich("質問", collections=["faq", "products", "manuals"])

# 全コレクションから検索
result = rag.enrich("質問")
```

### Node.js
```typescript
// 特定のコレクションから検索
const result = await rag.enrich('質問', { collection: 'faq' });

// 複数コレクションから検索
const result = await rag.enrich('質問', {
  collections: ['faq', 'products', 'manuals']
});

// 全コレクションから検索
const result = await rag.enrich('質問');
```

### Ruby
```ruby
# 特定のコレクションから検索
result = rag.enrich('質問', collection: 'faq')

# 複数コレクションから検索
result = rag.enrich('質問', collections: ['faq', 'products', 'manuals'])

# 全コレクションから検索
result = rag.enrich('質問')
```

---

## インストール

```bash
# Python
pip install rag-enricher

# Node.js / Next.js
npm install rag-enricher

# Ruby / Rails
gem install rag-enricher
```
