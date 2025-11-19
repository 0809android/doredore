# GitHubプッシュ前チェックリスト ✅

このドキュメントは、GitHubにコードをプッシュする前の最終確認リストです。

## 🔒 セキュリティチェック

### 1. 個人情報・認証情報
- [ ] APIキーやシークレットがハードコードされていない
- [ ] 環境変数から読み込む形式になっている
- [ ] `.env`ファイルが`.gitignore`に含まれている
- [ ] 個人のパス（`/Users/username/`など）が含まれていない

### 2. 除外ファイル確認
- [ ] `target/` ディレクトリが除外されている
- [ ] `*.db` ファイルが除外されている
- [ ] `.fastembed_cache/` が除外されている
- [ ] `.claude/` が除外されている
- [ ] `node_modules/` が除外されている

### 3. テストデータ
- [ ] テスト用データベースがコミットされていない
- [ ] デモ用CSVファイルがコミットされていない
- [ ] 一時ファイルが削除されている

## 📝 コード品質チェック

### 1. ビルド確認
```bash
# 全パッケージがビルドできることを確認
cargo check --workspace

# Pythonバインディング
cd rag-enricher-py && cargo check

# Node.jsバインディング
cd rag-enricher-js && cargo check

# Rubyバインディング
cd rag-enricher-rb && cargo check
```

### 2. テスト実行
```bash
# コアのテスト
cd rag-enricher-core && cargo test

# 検索モードテスト
cargo test test_three_search_modes
cargo test test_all_search_modes_english
```

### 3. フォーマット確認
```bash
cargo fmt --all -- --check
```

## 📚 ドキュメントチェック

- [ ] README.mdが最新の情報を反映している
- [ ] リポジトリURLが正しい
- [ ] インストール手順が正確
- [ ] 使用例が動作する
- [ ] ライセンス情報が記載されている

## 🔗 リポジトリ設定確認

### Cargo.toml
```toml
[workspace.package]
repository = "https://github.com/0809android/doredore"
license = "MIT"
```

### package.json (Node.js)
```json
{
  "repository": {
    "type": "git",
    "url": "https://github.com/0809android/doredore"
  }
}
```

## 🚀 プッシュ前最終確認

### 1. gitステータス確認
```bash
git status
```

### 2. 差分確認
```bash
git diff
```

### 3. コミット対象ファイル確認
```bash
git add -n .  # dry-run
```

### 4. 除外されるべきファイルの確認
```bash
# 以下のファイルは表示されないはず
git status --ignored | grep -E "(target/|\.db|\.fastembed_cache|\.claude|node_modules)"
```

## ✅ プッシュ手順

すべてのチェックが完了したら：

```bash
# 1. 変更をステージング
git add .

# 2. コミット
git commit -m "適切なコミットメッセージ"

# 3. リモートリポジトリ設定（初回のみ）
git remote add origin https://github.com/0809android/doredore.git

# 4. プッシュ
git push -u origin main
```

## 🎯 プッシュ後の確認

1. **GitHubでファイル確認**
   - README.mdが正しく表示されるか
   - 不要なファイルがプッシュされていないか
   - .gitignoreが機能しているか

2. **CIの確認**（設定している場合）
   - ビルドが成功するか
   - テストがパスするか

3. **READMEのリンク確認**
   - ドキュメントへのリンクが動作するか
   - バッジが正しく表示されるか

## ⚠️ よくある問題

### target/ディレクトリがプッシュされそうになる
```bash
# .gitignoreに追加されているか確認
cat .gitignore | grep target

# キャッシュをクリア
git rm -r --cached target/
```

### 個人情報を誤ってコミットした場合
```bash
# 最新のコミットを取り消す（プッシュ前のみ）
git reset --soft HEAD~1

# ファイルを修正してから再コミット
```

---

**重要**: このチェックリストは、公開リポジトリにプッシュする前に必ず確認してください！
