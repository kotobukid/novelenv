use clap::{Parser, Subcommand, Args};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, exit};

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
    }
}