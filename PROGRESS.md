# Rust学習 進捗管理

## 学習者プロフィール

| 項目 | 内容 |
|------|------|
| Java経験 | スクール講習を完走（基礎〜OOP理解済み） |
| Python経験 | Pyxelで軽いゲーム制作 |
| GAS経験 | 業務自動化 |
| Rust経験 | 入門レベル（少し触ったことがある程度） |
| 目標 | 段階を追って実務レベルへ到達 |
| 学習スタイル | 講義 → 設問 → 解説 の流れ |

---

## ロードマップと進捗

| Step | テーマ | 状態 | 完了日 |
|------|--------|------|--------|
| Step 1 | 基礎（変数・型・制御フロー・match） | ✅ 完了 | 2026-06-03 |
| Step 2 | 所有権・借用・可変参照 | ✅ 完了 | 2026-06-03 |
| Step 3 | データ構造（struct・enum・コレクション） | ✅ 完了 | 2026-06-03 |
| Step 4 | エラーハンドリング・モジュール | ✅ 完了 | 2026-06-03 |
| Step 5 | トレイト・ジェネリクス・テスト | ✅ 完了 | 2026-06-11 |
| Step 6 | 非同期・スマートポインタ・マクロ・最適化 | ✅ 完了 | 2026-06-15 |

---

## Step別 習得済みトピック詳細

### ✅ Step 1: 基礎
- `for` ループ（`1..=30` などの範囲）
- `match` とマッチアームガード（`_ if 条件`）
- FizzBuzz実装（matchを使った実践）

### ✅ Step 2: Rustの核心
- 所有権の移動（Move）と二重解放の防止
- `.clone()` によるヒープデータのコピー
- `&` による不変借用・`&mut` による可変借用
- 借用ルール（可変参照は同時に1つだけ）
- JavaのGCとRustのコンパイル時管理の違い

### ✅ Step 3: データ構造
- `struct` の定義・`impl` ブロック・`Self` キーワード・フィールド省略記法
- `enum` の定義（データを持つバリアント）と `match` の組み合わせ
- `match &self.field` による借用パターン
- `format!` でStringを返す設計
- `Vec`・`HashMap` の操作（`push`・`iter`・`for_each`・`entry().or_insert()`）
- `if let Some(...)` によるOption取り出し
- `fold` によるイテレータの集約（forループ版とfold版を両方実装）

### ✅ Step 4: エラーハンドリング・モジュール
- `Option<T>` / `Result<T, E>` の基本と使い分け
- `if let` / `match` / `.is_some()` / `any()` の使い分け
- `ok_or()` による `Option → Result` 変換
- `?` 演算子（エラー伝播・複数Resultの連鎖）
- モジュールシステム（`mod`・`pub`・ファイル分割）
- カプセル化（フィールドを非公開にしてメソッド経由でアクセス）

---

### ✅ Step 5: トレイト・ジェネリクス・テスト
- `#[cfg(test)]` / `#[test]` / `use super::*` の構造
- `assert_eq!` とカスタムメッセージによる検証
- テストヘルパー関数によるDRY（`make_test_book`）
- stringly-typed の問題と型安全な設計（`&str` → `Genre` への改善）
- 3パターン分岐の全ケースをカバーする網羅的テスト

---

## 直近の学習メモ

### 2026-06-03
- Step 1〜4 を一日で完走
- 学習スタイルを「講義 → 設問 → 解説」に統一
- ファイル分割（book.rs / library.rs / main.rs）を実践
- カプセル化・`?` 演算子・`any()` など実務的な概念を習得

### 2026-06-08
- Step 5 開始：トレイト（`trait`）の講義・実装
- `Printable` トレイトを `printable.rs` に定義（必須メソッド＋デフォルト実装）
- `impl Printable for Book` を実装（`category()`・`display()`・`full_info()` の設計議論を経て完成）
- 単一責任の原則を体感：セパレータのスペースは `full_info` に持たせるべきと自力で気づいた
- トレイト境界の2構文（`<T: Trait>` と `impl Trait`）を両方実装・違いを理解
- フリー関数の設計判断：`print_item` を `Library` の `impl` から `printable.rs` のフリー関数へ移動

### 2026-06-10
- ジェネリクス（`<T>`）の講義・実装
- `Library<T: Printable>` にジェネリクス化
- `impl<T: Printable> Library<T>`（汎用）と `impl Library<Book>`（Book専用）の分割パターンを習得
- 設計判断の基準「トレイトの保証だけで動くか否か」を自力で理解
- 不要インポートの警告を自分で気づいて修正

### 2026-06-25
- ミニプロジェクト Phase 3（save/load）・Phase 4（CLIコマンド解析）を実装（git: 「実装中」「load修正中」「load再修正」「ｃLIタスク実装」）
- Phase 3 は仕様のCSV保存から自主判断でJSON保存（`serde`/`serde_json`, `Box<dyn Error>`）に変更
- PROGRESS.md の更新が漏れていたため、次回セッション冒頭でレビューを実施する

### 2026-06-11
- 自動テスト（`#[test]`・`cargo test`）の講義・実装
- `Book` に5つのテストを実装（`test_book_new`・`display`・`category` 3パターン）
- テストヘルパー関数 `make_test_book(genre: Genre)` で DRY を実現
- stringly-typed の問題を自力で理解し、`Genre` 型を直接渡す型安全な設計に改善
- Step 5 完了

---

## 次回セッションの開始点

**総合ミニプロジェクト Phase 3・4 のレビュー → Phase 5（テスト）へ**

- 前回（2026-06-25）に Phase 3（save/load, serde_json化）と Phase 4（CLIコマンド解析）を実装済みだが、講師によるレビュー（解説ステップ）が未実施。
- まずは `task_manager.rs` と `main.rs` の実務観点レビューから再開する。
- レビュー後、Phase 5（`#[test]` によるユニットテスト）の講義・設問へ進む。

---

**Step 6: スマートポインタ・非同期・マクロ・最適化**（完了）

| トピック | 状態 |
|----------|------|
| `Box<T>` | ✅ 講義・設問・解説 完了（`src/list.rs`） |
| `Rc<T>` | ✅ 講義・設問・解説 完了（`src/shared.rs`） |
| `Rc<RefCell<T>>` | ✅ 講義・設問・解説 完了（`src/logger.rs`） |
| `Arc<T>` + `Mutex<T>` | ✅ 講義・設問・解説 完了（`src/counter.rs`） |
| `async/await`（Tokio） | ✅ 講義・設問・解説 完了（`src/async_task.rs`） |
| マクロ（`macro_rules!`） | ✅ 講義・設問・解説 完了（`src/macros.rs`） |

**Step 6 全トピック完了**

---

## 総合ミニプロジェクト：CLIタスク管理ツール

**開始日：** 2026-06-16
**ファイル構成：** `src/task.rs` / `src/task_manager.rs` / `src/main.rs`

### フェーズ一覧

| フェーズ | 内容 | 状態 |
|----------|------|------|
| Phase 1 | データ構造設計（`Task` struct / `Status` enum） | ✅ 完了 |
| Phase 2 | タスク管理ロジック（`TaskManager` / `add` / `list`） | ✅ 完了 |
| Phase 3 | ファイル保存・読み込み（`save` / `load` / `Result`） | ✅ 実装済み（未レビュー） |
| Phase 4 | CLIコマンド解析（`std::env::args` / `match`） | ✅ 実装済み（未レビュー） |
| Phase 5 | テスト（`#[test]`） | ⬜ 次回ここから |

### Phase 1 習得事項
- `u32` を ID に使う意味（符号なし整数で意味を型で表現）
- フィールドを非公開にして getter メソッドでカプセル化
- `title()` が `&str` を返す理由（clone 不要・借用で十分）

### Phase 2 習得事項
- 不要な `.clone()` を排除（`println!` が借用するだけ、という気づき）
- タスクを変数に一時保持してから push する設計（処理順序の意味を意識）
- `iter().for_each()` による簡潔なイテレーション

---

### Phase 3 設問（提出済み・レビュー未実施）

※実際の実装は仕様（CSV風テキスト）から逸脱し、`serde` / `serde_json` を使ったJSON保存に変更されている（`data.json`）。
エラー型も `Result<(), String>` ではなく `Result<(), Box<dyn Error>>` を採用。これは次回セッション冒頭でレビューする。

`task_manager.rs` に以下の2メソッドを追加する。

**保存フォーマット（CSV風テキスト）：**
```
1,牛乳を買う,todo
2,洗濯をする,done
3,掃除機をかける,todo
```

**1. `save(&self, path: &str) -> Result<(), String>`**
- `tasks` の内容を上記フォーマットで1行ずつファイルに書き出す
- 各行：`{id},{title},{status}`（status は `todo` / `done` の文字列）
- `std::fs::write` でファイルに書き込む
- エラー時は `.map_err(|e| e.to_string())` で `Err(String)` に変換して返す

**2. `load(&mut self, path: &str) -> Result<(), String>`**
- `std::path::Path::new(path).exists()` でファイル存在確認（なければ即 `Ok(())` を返す）
- `std::fs::read_to_string` でファイルを読む
- 1行ずつ `,` で分割して `Task` を復元する
- status 文字列 → `Status` の変換は `match` を使う
- `next_id` はロードしたタスクの最大ID + 1 にする
- `use crate::task::Status` を追加でインポートする必要あり

---

### Phase 4 設問（提出済み・レビュー未実施）

`main.rs` を書き換えて、コマンドライン引数でタスク管理ツールを操作できるようにする。

**操作コマンド：**
```bash
cargo run -- add "牛乳を買う"   # タスク追加
cargo run -- list               # 一覧表示
cargo run -- done 1             # ID=1 を完了にする
cargo run -- delete 1           # ID=1 を削除する
```

**実装ポイント：**
- `std::env::args()` でコマンドライン引数を取得
- `match` でサブコマンド（`add` / `list` / `done` / `delete`）を振り分け
- `TaskManager` を生成 → `load` → 操作 → `save` の流れ
- 保存ファイルは `tasks.csv` などの固定パスでよい
- `done` コマンド用に `TaskManager::complete(id: u32)` メソッドも追加が必要
- `delete` コマンド用に `TaskManager::delete(id: u32)` メソッドも追加が必要

---

### Phase 5 設問（未着手）

`task_manager.rs` または `task.rs` に `#[cfg(test)]` ブロックを追加してユニットテストを書く。

**テスト対象（最低限）：**
1. `add` → `list` でタスクが増えることを確認
2. `complete` でステータスが `Done` に変わることを確認
3. `delete` でタスクが消えることを確認
4. `save` → `load` でデータが往復して復元されることを確認（ファイルI/Oの統合テスト）

---

### Phase 5 以降の発展トピック（参考）

ミニプロジェクト完了後に興味があれば取り組める発展テーマ：

| テーマ | 内容 | 難易度 |
|--------|------|--------|
| JSON対応 | `serde` / `serde_json` クレートでJSONシリアライズ | ★★☆ |
| エラー型の強化 | `thiserror` クレートでカスタムエラー型を定義 | ★★☆ |
| Web API化 | `axum` でREST APIサーバーに発展 | ★★★ |
| 非同期ファイルI/O | `tokio::fs` で `async fn save/load` に書き換え | ★★☆ |
| 手続きマクロ | `derive` マクロを自作する | ★★★ |

---

### Step 6 設問（提出済み・実装確認済み）：料理シミュレーター（async/await）

`src/async_task.rs` を新規作成し、以下を実装する：

1. `async fn boil_water() -> &'static str` → 2秒待って "お湯が沸いた" を返す
2. `async fn chop_vegetables() -> &'static str` → 1秒待って "野菜が切れた" を返す
3. `async fn prepare_sauce() -> &'static str` → 3秒待って "ソースができた" を返す
4. `pub async fn run()` → 3つを `tokio::join!` で並列実行し、結果と経過時間を表示

**待ち時間：** `tokio::time::sleep(tokio::time::Duration::from_secs(N)).await;`
**時刻計測：** `let start = std::time::Instant::now();` / `start.elapsed()`
**Cargo.toml に追加：** `tokio = { version = "1", features = ["full"] }`
**main.rs の変更：** `#[tokio::main]` を付けて `async fn main()` にし、`async_task::run().await;` を呼ぶ

**期待出力：** 経過時間が「約3秒台」（直列なら6秒かかるところを並列で短縮）

---

### Step 6 講義メモ：スマートポインタ全体像

| 型 | 用途 | スレッド |
|----|------|---------|
| `Box<T>` | ヒープ確保・再帰構造 | 単一所有 |
| `Rc<T>` | 複数所有者 | シングルスレッドのみ |
| `Arc<T>` | 複数所有者 | マルチスレッド対応 |
| `RefCell<T>` | 実行時借用チェック・内部可変性 | シングルスレッドのみ |
| `Mutex<T>` | 相互排他ロック | マルチスレッド対応 |

**組み合わせパターン：**
- `Rc<RefCell<T>>` → シングルスレッドで複数所有者が変更
- `Arc<Mutex<T>>` → マルチスレッドで複数所有者が変更

**`Arc<Mutex<T>>` の使い方：**
- `Arc::clone` はスレッドに渡す直前（ループ内）で呼ぶ
- `.lock().unwrap()` でロック取得 → `MutexGuard` がスコープ終了で自動解放
- `thread::spawn(move || { ... })` の `move` でクロージャに所有権を移す
- `handle.join().unwrap()` でスレッドの終了を待つ

### Step 6 講義メモ：async/await

**なぜ非同期か：**
- スレッド → OS レベルの並列（CPU バウンド向き）
- async/await → IO 待ち中に別タスクへ制御を渡す（IO バウンド向き）

**`async fn` は `Future` を返す：**
- 呼び出しただけでは実行されない。ランタイムが `.await` で駆動する
- Rust 標準ライブラリにランタイムは**ない** → `tokio` クレートを使う

**基本構文：**
- `#[tokio::main]` → `async fn main()` をランタイムで動かすマクロ
- `tokio::join!(a(), b())` → 複数 Future を並列実行し全部完了を待つ
- `tokio::time::sleep(...).await` → 非同期の待機（スレッドをブロックしない）

**直列 vs 並列：**
- `.await` を逐次書く → 直列（合計時間 = 各タスクの和）
- `tokio::join!` → 並列（合計時間 = 最長タスクのみ）
