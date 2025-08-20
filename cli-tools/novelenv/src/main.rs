use clap::{Parser, Subcommand, Args};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "novel")]
#[command(about = "NovelEnv - Novel writing environment management tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize a new novel project")]
    Init(InitArgs),
    #[command(about = "Activate novel environment (use 'source <(novel activate)')")]
    Activate,
    #[command(about = "Find context information")]
    FindContext(FindContextArgs),
    #[command(about = "Context weaver operations")]
    Weave(WeaveArgs),
    #[command(about = "Dump episode information")]
    Dump(DumpArgs),
    #[command(about = "Pick a random character name")]
    PickName(PickNameArgs),
    #[command(about = "Manage writing styles")]
    Style(StyleArgs),
    #[command(about = "Analyze text style and statistics")]
    Profile(ProfileArgs),
}

#[derive(Args)]
struct InitArgs {
    #[arg(help = "Project name")]
    name: String,
    #[arg(help = "Additional arguments")]
    args: Vec<String>,
}

#[derive(Args)]
struct FindContextArgs {
    #[arg(help = "Subcommand (profile, episode)")]
    subcommand: String,
    #[arg(help = "Additional arguments")]
    args: Vec<String>,
}

#[derive(Args)]
struct WeaveArgs {
    #[arg(help = "Subcommand (serve, resolve)")]
    subcommand: String,
    #[arg(help = "Additional arguments")]
    args: Vec<String>,
}

#[derive(Args)]
struct DumpArgs {
    #[arg(help = "What to dump (episodes)")]
    target: String,
}

#[derive(Args)]
struct PickNameArgs {
    #[arg(help = "Additional arguments (--genre, --gender)")]
    args: Vec<String>,
}

#[derive(Args)]
struct StyleArgs {
    #[command(subcommand)]
    command: StyleCommands,
}

#[derive(Args)]
struct ProfileArgs {
    #[arg(help = "Additional arguments")]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum StyleCommands {
    #[command(about = "List available and installed writing styles")]
    List,
    #[command(about = "Install a writing style to the current project")]
    Install {
        #[arg(help = "Name of the style to install (without .md extension)")]
        name: String,
        #[arg(long, help = "Install to local project (default)")]
        local: bool,
    },
    #[command(about = "Show information about a writing style")]
    Info {
        #[arg(help = "Name of the style (without .md extension)")]
        name: String,
    },
}

fn get_tool_path(tool_name: &str) -> PathBuf {
    // 現在の実行ファイルの場所を取得
    let current_exe = env::current_exe()
        .expect("Failed to get current executable path");
    
    let exe_dir = current_exe.parent()
        .expect("Failed to get executable directory");
    
    // まず同じディレクトリで探す（インストール済み環境）
    let installed_path = exe_dir.join(tool_name);
    if installed_path.exists() {
        return installed_path;
    }
    
    // 開発環境のパスを試す
    // 想定構造: cli-tools/novelenv/target/release/novel
    //          cli-tools/find-context/target/release/find-context
    if let Some(cli_tools_dir) = exe_dir
        .parent() // target
        .and_then(|p| p.parent()) // release
        .and_then(|p| p.parent()) // novelenv
    {
        let dev_path = match tool_name {
            "novel-init" => cli_tools_dir
                .join("novel-init")
                .join("target")
                .join("release")
                .join("novel-init"),
            "find-context" => cli_tools_dir
                .join("find-context")
                .join("target")
                .join("release")
                .join("find-context"),
            "weaver" => cli_tools_dir
                .join("context-weaver")
                .join("target")
                .join("release")
                .join("weaver"),
            "dump-episode-info" => cli_tools_dir
                .join("dump-episode-info")
                .join("target")
                .join("release")
                .join("dump-episode-info"),
            "pick-name" => cli_tools_dir
                .join("pick-name")
                .join("target")
                .join("release")
                .join("pick-name"),
            "profile" => cli_tools_dir
                .join("profile")
                .join("target")
                .join("release")
                .join("profile"),
            _ => panic!("Unknown tool: {}", tool_name),
        };
        
        if dev_path.exists() {
            return dev_path;
        }
    }
    
    // どちらでも見つからない場合はエラー
    panic!("Tool '{}' not found in {} or development paths", tool_name, exe_dir.display());
}

fn execute_tool(tool_path: PathBuf, args: Vec<String>) {
    let status = Command::new(&tool_path)
        .args(&args)
        .status();
    
    match status {
        Ok(exit_status) => {
            if let Some(code) = exit_status.code() {
                exit(code);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute {}: {}", tool_path.display(), e);
            eprintln!("Make sure the tool is built with: cargo build --release");
            exit(1);
        }
    }
}

fn find_project_root() -> Option<(PathBuf, String)> {
    let current_dir = env::current_dir().ok()?;
    
    // 現在のディレクトリから上に向かって.novelenvディレクトリを探す
    let mut dir = current_dir.as_path();
    loop {
        let novelenv_dir = dir.join(".novelenv");
        if novelenv_dir.exists() && novelenv_dir.is_dir() {
            let project_name = dir.file_name()?.to_string_lossy().to_string();
            return Some((dir.to_path_buf(), project_name));
        }
        
        dir = dir.parent()?;
    }
}

fn activate_environment() {
    if let Some((project_root, project_name)) = find_project_root() {
        println!("# NovelEnv activation script");
        println!("export NOVELENV_ACTIVE=\"{}\"", project_name);
        println!("export NOVELENV_PROJECT_ROOT=\"{}\"", project_root.display());
        
        // プロンプト設定（bashとzsh対応）
        println!("if [ -n \"$BASH_VERSION\" ]; then");
        println!("  export PS1=\"({}) $PS1\"", project_name);
        println!("elif [ -n \"$ZSH_VERSION\" ]; then");
        println!("  export PS1=\"({}) %{{$reset_color%}}$PS1\"", project_name);
        println!("fi");
        
        println!("echo \"📝 NovelEnv activated: {}\"", project_name);
    } else {
        eprintln!("❌ NovelEnvプロジェクトが見つかりません");
        eprintln!("   プロジェクトディレクトリ内で実行するか、novel init でプロジェクトを作成してください");
        exit(1);
    }
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init(args) => {
            let tool_path = get_tool_path("novel-init");
            let mut cmd_args = vec![args.name];
            cmd_args.extend(args.args);
            execute_tool(tool_path, cmd_args);
        }
        Commands::Activate => {
            activate_environment();
        }
        Commands::FindContext(args) => {
            let tool_path = get_tool_path("find-context");
            let mut cmd_args = vec![args.subcommand];
            cmd_args.extend(args.args);
            execute_tool(tool_path, cmd_args);
        }
        Commands::Weave(args) => {
            let tool_path = get_tool_path("weaver");
            let mut cmd_args = vec![args.subcommand];
            cmd_args.extend(args.args);
            execute_tool(tool_path, cmd_args);
        }
        Commands::Dump(args) => {
            let tool_path = get_tool_path("dump-episode-info");
            // dump-episode-info は引数なしで実行
            execute_tool(tool_path, vec![]);
        }
        Commands::PickName(args) => {
            let tool_path = get_tool_path("pick-name");
            execute_tool(tool_path, args.args);
        }
        Commands::Style(args) => {
            handle_style_command(args);
        }
        Commands::Profile(args) => {
            let tool_path = get_tool_path("profile");
            execute_tool(tool_path, args.args);
        }
    }
}

fn handle_style_command(args: StyleArgs) {
    match args.command {
        StyleCommands::List => {
            list_styles();
        }
        StyleCommands::Install { name, local: _ } => {
            install_style(&name);
        }
        StyleCommands::Info { name } => {
            show_style_info(&name);
        }
    }
}

fn find_novelenv_writing_style_dir() -> Option<PathBuf> {
    // NovelEnvのwriting_styleディレクトリを探す
    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            // 開発環境のパスを試す
            // /home/kotobukid/projects/novelenv/cli-tools/novelenv/target/release/novel から遡る
            // novel -> release -> target -> novelenv -> cli-tools -> novelenv(root)
            let mut current = exe_dir;
            for _ in 0..4 {
                current = current.parent()?;
            }
            let dev_path = current.join("writing_style");

            if dev_path.exists() {
                return Some(dev_path);
            }

            // インストール済み環境のパスを試す
            // ~/.local/bin/novel から探す
            if let Some(parent) = exe_dir.parent() {
                if let Some(grandparent) = parent.parent() {
                    let installed_path = grandparent
                        .join("projects")
                        .join("novelenv")
                        .join("writing_style");

                    if installed_path.exists() {
                        return Some(installed_path);
                    }
                }
            }
        }
    }

    None
}

fn get_local_writing_style_dir() -> Option<PathBuf> {
    // 現在のディレクトリから上に向かってwriting_styleディレクトリを探す
    let current_dir = env::current_dir().ok()?;
    let mut dir = current_dir.as_path();

    loop {
        let writing_style_dir = dir.join("writing_style");
        let novelenv_dir = dir.join(".novelenv");

        // .novelenvディレクトリがあり、writing_styleディレクトリも存在する場合
        if novelenv_dir.exists() && writing_style_dir.exists() {
            return Some(writing_style_dir);
        }

        dir = dir.parent()?;
    }
}

fn list_styles() {
    let global_dir = find_novelenv_writing_style_dir();
    let local_dir = get_local_writing_style_dir();

    if global_dir.is_none() {
        eprintln!("❌ NovelEnvのwriting_styleディレクトリが見つかりません");
        exit(1);
    }

    if local_dir.is_none() {
        eprintln!("❌ NovelEnvプロジェクトディレクトリ内で実行してください");
        exit(1);
    }

    let global_dir = global_dir.unwrap();
    let local_dir = local_dir.unwrap();

    // グローバルとローカルのスタイルを収集
    let global_styles = collect_style_files(&global_dir);
    let local_styles = collect_style_files(&local_dir);

    println!("📝 Writing Styles:");
    println!();

    // すべてのスタイルをソートして表示
    let mut all_styles: Vec<_> = global_styles.union(&local_styles).collect();
    all_styles.sort();

    for style in all_styles {
        let is_installed = local_styles.contains(style);
        let status = if is_installed {
            "✅"
        } else {
            "🆕"
        };

        let display_name = style.trim_end_matches(".md");
        println!("{} {} {}",
            status,
            display_name,
            if is_installed { "(installed)" } else { "(available)" }
        );
    }

    println!();
    println!("💡 新しいスタイルをインストールするには:");
    println!("   novel style install <style_name>");
}

fn collect_style_files(dir: &Path) -> HashSet<String> {
    let mut styles = HashSet::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    if let Some(filename) = path.file_name() {
                        styles.insert(filename.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    styles
}

fn install_style(name: &str) {
    let global_dir = find_novelenv_writing_style_dir();
    let local_dir = get_local_writing_style_dir();

    if global_dir.is_none() {
        eprintln!("❌ NovelEnvのwriting_styleディレクトリが見つかりません");
        exit(1);
    }

    if local_dir.is_none() {
        eprintln!("❌ NovelEnvプロジェクトディレクトリ内で実行してください");
        exit(1);
    }

    let global_dir = global_dir.unwrap();
    let local_dir = local_dir.unwrap();

    // .mdを付けてファイル名を構築
    let filename = if name.ends_with(".md") {
        name.to_string()
    } else {
        format!("{}.md", name)
    };

    let source_path = global_dir.join(&filename);
    let dest_path = local_dir.join(&filename);

    // ソースファイルの存在確認
    if !source_path.exists() {
        eprintln!("❌ スタイル '{}' が見つかりません", name);
        eprintln!("   利用可能なスタイルを確認するには: novel style list");
        exit(1);
    }

    // 既にインストール済みかチェック
    if dest_path.exists() {
        eprintln!("⚠️  スタイル '{}' は既にインストールされています", name);
        exit(1);
    }

    // ファイルをコピー
    match fs::copy(&source_path, &dest_path) {
        Ok(_) => {
            println!("✅ スタイル '{}' をインストールしました", name);
            println!("   場所: {}", dest_path.display());
        }
        Err(e) => {
            eprintln!("❌ スタイルのインストールに失敗しました: {}", e);
            exit(1);
        }
    }
}

fn show_style_info(name: &str) {
    let global_dir = find_novelenv_writing_style_dir();
    let local_dir = get_local_writing_style_dir();

    // .mdを付けてファイル名を構築
    let filename = if name.ends_with(".md") {
        name.to_string()
    } else {
        format!("{}.md", name)
    };

    // ローカルを優先して探す
    let mut style_path = None;
    let mut location = "";

    if let Some(ref local) = local_dir {
        let local_path = local.join(&filename);
        if local_path.exists() {
            style_path = Some(local_path);
            location = " (local)";
        }
    }

    // ローカルになければグローバルを探す
    if style_path.is_none() {
        if let Some(ref global) = global_dir {
            let global_path = global.join(&filename);
            if global_path.exists() {
                style_path = Some(global_path);
                location = " (global)";
            }
        }
    }

    match style_path {
        Some(path) => {
            println!("📝 スタイル情報: {}{}", name, location);
            println!();

            // ファイルの最初の20行を読み取って表示
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().take(20).collect();
                    for line in lines {
                        println!("{}", line);
                    }

                    if content.lines().count() > 20 {
                        println!("\n... (以下省略)");
                    }
                }
                Err(e) => {
                    eprintln!("❌ ファイルの読み取りに失敗しました: {}", e);
                    exit(1);
                }
            }
        }
        None => {
            eprintln!("❌ スタイル '{}' が見つかりません", name);
            eprintln!("   利用可能なスタイルを確認するには: novel style list");
            exit(1);
        }
    }
}