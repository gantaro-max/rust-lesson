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

**総合ミニプロジェクト（CLIタスク管理ツール）Phase 1〜5 すべて完了。次のテーマを相談して決める。**

- Phase 5（ユニットテスト）は 2026-07-15 に完了。`add`/`complete`/`delete`/`save`→`load`の4テスト全て実装・パス。
- 次回セッション冒頭で、以下のような発展テーマから何をやるか相談する：
  - PROGRESS.md記載の「Phase 5以降の発展トピック」（`clap`によるCLI再実装、`thiserror`、`axum`でWeb API化、`tokio::fs`で非同期I/O化、手続きマクロ自作）
  - 新しい題材でのミニプロジェクト
  - 実務でよく使う周辺知識（`Iterator`トレイトの自作、`From`/`Into`変換、ライフタイム注釈の実践など）

### 2026-07-15 セッションメモ（Phase 5：ユニットテスト）

- `Status`/`Task`に`PartialEq`を`derive`追加、`Task`に`status()`ゲッターを追加（カプセル化を保ったままテストで状態を検証するため）
- テスト1（`add`→増加確認）：`Vec`の0-indexedを勘違いして`.get(1)`で`None`を引いた実機エラーから、添字の考え方を再確認
- テスト2（`complete`→`Done`確認）：最初`add`を使ってセットアップしていたが、「テスト同士を独立させたい」と自ら判断し、`Task::new`を直接`tasks`にpushする設計に自主的にリファクタリング
- テスト3（`delete`→消える確認）：同じ独立設計パターンをスムーズに横展開できた
- `status()`が本体コードから未使用で`dead_code`警告が出た件：`_status()`へのリネームで警告を消す案を一度試したが、「名前の意味と意図の矛盾」を指摘され、最終的に「警告を許容する」という自覚的な判断に着地（`#[allow(dead_code)]`という選択肢も紹介済み）
- テスト4（`save`→`load`往復）：最初`Arc<Mutex<TaskManager>>` + `thread::spawn`で実装しようとして迷走 → 「cargo testの並列実行」という説明の言葉選びが誤解を招いたと本人からフィードバックあり（→ memory: feedback_teaching_phrasing に記録）。最終的にインスタンスを分離し`.ok()`を`.unwrap()`に修正。「`load`をわざと壊しても今のテストは本当に落ちるか」を実際に確認し、テストの実効性を検証する体験をした

### 2026-07-14 セッションメモ

- Phase 3・4 のレビュー実施（`task_manager.rs` / `main.rs`）
  - `args[2]` の直接インデックスアクセス（パニックの危険）→ `args.get(2)` へ自主修正済みだったのを確認・レビュー
  - `delete` の `.filter().cloned().collect()`（無駄なクローン）→ `.retain()` へ自主修正済みだったのを確認・レビュー
  - `Index`（`[]`）と`.get()`の設計思想の違い（「プログラムの前提崩壊=バグはpanic」「外部由来の正常な失敗はOption/Result」）を対話で理解
- `main.rs` のネスト解消リファクタリングを実施
  - `Option::and_then`で試行 → `.ok()`でエラー情報を握り潰す失敗を経験（`Result::ok()`の挙動を実機で確認）
  - `Option::ok_or_else` → `Result::and_then` → `map_err` のチェーンに書き換え、`Result<u32, String>`に統一する設計に到達
  - 一度`println!`をコンビネータの中に埋め込んでしまい、エラーメッセージが二重表示されるバグを実際に`cargo run`で確認 → 「コンビネータは副作用ではなくデータを返す場所」という原則を学習
  - `"done"`/`"delete"`の重複ロジックを`parse_task_id(args: &[String]) -> Result<u32, String>`として関数抽出（DRY）
  - 引数を`Vec<String>`（所有権ごと）ではなく`&[String]`（借用）にすべきと指摘 → 自力で`&[String]`に修正（`&Vec<T>`より`&[T]`が柔軟という理由も踏まえて選択）
  - `"add"`には同じリファクタリングが不要な理由（失敗ポイントが1つしかない）を自力で説明できた
- 副次的に `use std::f32::consts::E;` という誤爆import（IDE自動補完由来と推測）にも気づき、警告ゼロの状態を維持する習慣を確認

### Phase 3・4 総括（実装内容）

- `task_manager.rs`：`save`/`load`をJSON（`serde_json`）で実装、`complete`/`delete`メソッド追加
- `main.rs`：`std::env::args()` + `match`によるサブコマンド解析（`add`/`list`/`done`/`delete`）、`parse_task_id`ヘルパー関数によるDRY化

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
| Phase 3 | ファイル保存・読み込み（`save` / `load` / `Result`） | ✅ 完了（レビュー済み） |
| Phase 4 | CLIコマンド解析（`std::env::args` / `match`） | ✅ 完了（レビュー済み） |
| Phase 5 | テスト（`#[test]`） | ✅ 完了 |

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
