use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// VectCut API 客户端
///
/// 封装对 VectCutAPI HTTP 服务的调用
pub struct VectCutClient {
    base_url: String,
    client: Client,
    draft_id: Option<String>,
}

impl VectCutClient {
    /// 创建新的客户端实例
    ///
    /// # Arguments
    /// * `base_url` - VectCutAPI 服务地址，默认 "http://127.0.0.1:9001"
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            draft_id: None,
        }
    }

    /// 创建默认客户端（localhost:9001）
    pub fn default() -> Self {
        Self::new("http://127.0.0.1:9001")
    }

    /// 创建新草稿
    ///
    /// # Arguments
    /// * `width` - 视频宽度（如 1080）
    /// * `height` - 视频高度（如 1920）
    pub async fn create_draft(&mut self, width: u32, height: u32) -> Result<String> {
        #[derive(Serialize)]
        struct CreateDraftRequest {
            canvas_width: u32,
            canvas_height: u32,
        }

        #[derive(Deserialize)]
        struct CreateDraftResponse {
            draft_id: String,
        }

        let url = format!("{}/create_draft", self.base_url);
        let req = CreateDraftRequest {
            canvas_width: width,
            canvas_height: height,
        };

        let resp: CreateDraftResponse = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        self.draft_id = Some(resp.draft_id.clone());
        tracing::info!("草稿创建成功: {}", resp.draft_id);
        Ok(resp.draft_id)
    }

    /// 添加视频素材
    ///
    /// # Arguments
    /// * `video_url` - 视频 URL 或本地路径
    /// * `start` - 开始时间（秒）
    /// * `end` - 结束时间（秒）
    /// * `volume` - 音量（0.0-1.0）
    pub async fn add_video(
        &self,
        video_url: impl AsRef<str>,
        start: f64,
        end: f64,
        volume: f64,
    ) -> Result<()> {
        let draft_id = self.draft_id.as_ref().ok_or(anyhow!("未创建草稿"))?;

        #[derive(Serialize)]
        struct AddVideoRequest<'a> {
            draft_id: &'a str,
            video_url: &'a str,
            start: f64,
            #[serde(rename = "end")]
            end_time: f64,
            volume: f64,
        }

        let url = format!("{}/add_video", self.base_url);
        let req = AddVideoRequest {
            draft_id,
            video_url: video_url.as_ref(),
            start,
            end_time: end,
            volume,
        };

        self.client.post(&url).json(&req).send().await?;
        tracing::info!("视频添加成功: {}", video_url.as_ref());
        Ok(())
    }

    /// 添加文本
    ///
    /// # Arguments
    /// * `text` - 文本内容
    /// * `start` - 开始时间（秒）
    /// * `end` - 结束时间（秒）
    pub async fn add_text(
        &self,
        text: impl AsRef<str>,
        start: f64,
        end: f64,
    ) -> Result<()> {
        let draft_id = self.draft_id.as_ref().ok_or(anyhow!("未创建草稿"))?;

        #[derive(Serialize)]
        struct AddTextRequest<'a> {
            draft_id: &'a str,
            text: &'a str,
            start: f64,
            #[serde(rename = "end")]
            end_time: f64,
        }

        let url = format!("{}/add_text", self.base_url);
        let req = AddTextRequest {
            draft_id,
            text: text.as_ref(),
            start,
            end_time: end,
        };

        self.client.post(&url).json(&req).send().await?;
        tracing::info!("文本添加成功: {}", text.as_ref());
        Ok(())
    }

    /// 保存草稿
    ///
    /// 生成剪映可导入的草稿文件
    pub async fn save_draft(&self) -> Result<String> {
        let draft_id = self.draft_id.as_ref().ok_or(anyhow!("未创建草稿"))?;

        #[derive(Serialize)]
        struct SaveDraftRequest<'a> {
            draft_id: &'a str,
        }

        #[derive(Deserialize)]
        struct SaveDraftResponse {
            draft_path: String,
        }

        let url = format!("{}/save_draft", self.base_url);
        let req = SaveDraftRequest { draft_id };

        let resp: SaveDraftResponse = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        tracing::info!("草稿保存成功: {}", resp.draft_path);
        Ok(resp.draft_path)
    }
}
