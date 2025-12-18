use clap::{Parser, Subcommand};
use rrdc::Result;

mod command;

#[derive(Parser)]
#[command(name = "rrdc")]
#[command(about = "Release Date Crawler - 発売日情報の取得・登録")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// URLからHTMLを取得してstdoutに出力
    Fetch {
        #[arg(short, long, env = "TARGET_URL")]
        url: String,
    },
    /// stdinのHTMLをパースしてJSON出力
    Parse,
    /// stdinのJSONとDynamoDB既存データを比較してCalendar登録/更新
    Sync,
    /// 全ステップを実行 (Lambda用)
    Run,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { url } => command::fetch::execute(&url).await,
        Commands::Parse => command::parse::execute().await,
        Commands::Sync => command::sync::execute().await,
        Commands::Run => command::run::execute().await,
    }
}
