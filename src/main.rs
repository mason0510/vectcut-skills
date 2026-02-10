mod client;

use anyhow::Result;
use clap::{Parser, Subcommand};
use client::VectCutClient;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "vectcut")]
#[command(about = "VectCut API CLI - 自动化剪映视频编辑工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 创建简单的视频草稿
    Create {
        /// 视频文件路径
        #[arg(short, long)]
        video: String,

        /// 标题文本
        #[arg(short, long)]
        title: Option<String>,

        /// VectCut API 服务地址
        #[arg(short, long, default_value = "http://127.0.0.1:9001")]
        server: String,
    },

    /// 测试 API 连接
    Test {
        /// VectCut API 服务地址
        #[arg(short, long, default_value = "http://127.0.0.1:9001")]
        server: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create { video, title, server } => {
            create_video(&server, &video, title.as_deref()).await?
        }
        Commands::Test { server } => {
            test_connection(&server).await?
        }
    }

    Ok(())
}

/// 创建视频草稿
async fn create_video(server: &str, video_path: &str, title: Option<&str>) -> Result<()> {
    println!("🎬 正在创建视频草稿...");
    println!("   视频: {}", video_path);
    println!("   标题: {}", title.unwrap_or("(无)"));

    let mut client = VectCutClient::new(server);

    // 1. 创建草稿 (1080x1920 竖屏)
    println!("\n1️⃣  创建草稿...");
    let draft_id = client.create_draft(1080, 1920).await?;
    println!("   ✅ 草稿 ID: {}", draft_id);

    // 2. 添加视频
    println!("\n2️⃣  添加视频素材...");
    client.add_video(video_path, 0.0, 10.0, 1.0).await?;
    println!("   ✅ 视频已添加");

    // 3. 添加标题（如果有）
    if let Some(text) = title {
        println!("\n3️⃣  添加标题文本...");
        client.add_text(text, 0.0, 3.0).await?;
        println!("   ✅ 标题已添加: {}", text);
    }

    // 4. 保存草稿
    println!("\n4️⃣  保存草稿...");
    let draft_path = client.save_draft().await?;
    println!("   ✅ 草稿已保存: {}", draft_path);

    println!("\n✨ 完成！请在剪映中导入草稿进行编辑。");

    Ok(())
}

/// 测试 API 连接
async fn test_connection(server: &str) -> Result<()> {
    println!("🔍 测试 VectCut API 连接...");
    println!("   服务器: {}", server);

    let client = reqwest::Client::new();
    let url = format!("{}/create_draft", server);

    match client.post(&url).json(&serde_json::json!({})).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("   ✅ 连接成功！");
            } else {
                println!("   ⚠️  服务器返回错误: {}", resp.status());
            }
        }
        Err(e) => {
            println!("   ❌ 连接失败: {}", e);
            println!("\n💡 请确保 VectCutAPI 服务已启动：");
            println!("   cd VectCutAPI && python capcut_server.py");
        }
    }

    Ok(())
}
