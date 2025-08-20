# 設計メモ

## novelenv 統合

`novel profile` コマンドとして呼び出し可能にする：

```rust
// novelenv/src/main.rs への追加
enum Commands {
    // ...
    #[command(about = "Analyze text style and statistics")]
    Profile(ProfileArgs),
}

// profile は find-context と同様のパターンで実装
```

## CLIインターフェース

```rust
use clap::{Parser, ArgEnum};

#[derive(Parser)]
#[command(name = "profile")]
#[command(about = "Analyze text style and statistics")]
struct Cli {
    /// Input file path (or stdin if not specified)
    input: Option<PathBuf>,
    
    /// Compare with another file
    #[arg(short, long)]
    compare: Option<PathBuf>,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "terminal")]
    format: OutputFormat,
    
    /// Character encoding (for file input)
    #[arg(short = 'e', long, default_value = "utf-8")]
    encoding: String,
    
    /// Generate chart visualization
    #[arg(long)]
    chart: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Clone, ArgEnum)]
enum OutputFormat {
    Terminal,
    Json,
    Csv,
}
```

## アーキテクチャ

```
src/
├── main.rs           # CLIエントリポイント
├── analyzer/
│   ├── mod.rs
│   ├── dialogue.rs   # 会話文抽出
│   ├── sentence.rs   # 文境界検出
│   ├── rhythm.rs     # リズム分析
│   └── stats.rs      # 基本統計
├── parser/
│   ├── mod.rs
│   └── brackets.rs   # nom による括弧パース
├── report/
│   ├── mod.rs
│   ├── display.rs    # ターミナル表示
│   ├── chart.rs      # ASCIIチャート
│   └── json.rs       # JSON出力
├── encoding.rs       # 文字エンコーディング処理
└── models.rs         # データ構造定義
```

## データフロー

```
入力テキスト
    ↓
[前処理] 正規化（改行統一、空白処理）
    ↓
[分析器群] 並列実行
    ├→ 会話文抽出
    ├→ 文分割
    ├→ 文字種カウント
    └→ 段落解析
    ↓
[集計] AnalysisResult 構造体に集約
    ↓
[出力] フォーマット選択
    ├→ ターミナル（デフォルト）
    ├→ JSON
    └→ 比較モード
```

## コア構造体

```rust
pub struct TextProfile {
    pub basic_stats: BasicStats,
    pub dialogue_stats: DialogueStats,
    pub rhythm_metrics: RhythmMetrics,
    pub readability: ReadabilityScore,
}

pub struct BasicStats {
    pub total_chars: usize,
    pub total_sentences: usize,
    pub total_paragraphs: usize,
    pub char_types: CharTypeDistribution,
}

pub struct DialogueStats {
    pub dialogue_chars: usize,
    pub narrative_chars: usize,
    pub dialogue_ratio: f32,
    pub dialogue_segments: Vec<TextSegment>,
}

pub struct RhythmMetrics {
    pub sentence_lengths: Vec<usize>,
    pub avg_sentence_length: f32,
    pub variation_coefficient: f32,  // ばらつき度
    pub tempo_pattern: TempoPattern,  // 短長短長のパターン
}
```

## nom パーサー設計

```rust
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    sequence::{delimited, pair},
    multi::many0,
    combinator::{map, recognize},
};

// 基本的な会話文パーサー
fn dialogue_basic(input: &str) -> IResult<&str, Dialogue> {
    delimited(
        tag("「"),
        take_until("」"),
        tag("」")
    )(input)
}

// 入れ子対応版（Phase 2）
fn dialogue_nested(input: &str) -> IResult<&str, Dialogue> {
    fn inner(input: &str) -> IResult<&str, String> {
        // 『』の中の「」も正しく処理
        alt((
            map(
                delimited(tag("『"), inner_content, tag("』")),
                |s| format!("『{}』", s)
            ),
            map(
                take_while(|c| c != '」' && c != '『'),
                String::from
            ),
        ))(input)
    }
    
    delimited(
        tag("「"),
        inner,
        tag("」")
    )(input)
}
```

## パフォーマンス目標

- 1万字のテキスト: < 100ms
- 10万字のテキスト: < 1s
- メモリ使用量: < 50MB

## エラーハンドリング

```rust
pub enum ProfileError {
    IoError(std::io::Error),
    ParseError(String),
    InvalidInput(String),
}

// 括弧が閉じていない場合
// → 警告を出すが処理は継続
// → 該当部分は地の文として扱う

// 文字エンコーディング問題
// → UTF-8 のみサポート
// → それ以外はエラー
```

## テスト戦略

```rust
#[cfg(test)]
mod tests {
    // ユニットテスト: 各パーサーの動作
    // 統合テスト: 実際の小説テキストサンプル
    // ベンチマーク: 1万字での処理速度
    
    #[test]
    fn test_nested_dialogue() {
        let text = "「彼が『まさか！』と言った」";
        // 正しく入れ子を処理できるか
    }
    
    #[test]
    fn test_unclosed_bracket() {
        let text = "「これは閉じてない";
        // エラーではなく警告として処理
    }
}
```

## 将来の拡張性

- プラグイン機構（カスタム分析器の追加）
- Web UI版への対応（wasm-pack）
- 他ツールとの連携（context-weaver の結果を分析）

## 参考: 他ツールとの差別化

- find-context: 検索・取得に特化
- dump-episode-info: メタデータ抽出に特化
- **profile: 文体分析に特化** ← New!

各ツールが単一責任を持つことで、組み合わせて使える。