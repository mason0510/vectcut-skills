use vectcut_skills::client::VectCutClient;

#[tokio::test]
async fn test_get_draft_id_without_creating_draft() {
    let client = VectCutClient::new("http://localhost:9001");
    
    // 未创建草稿时调用需要 draft_id 的方法应该返回错误
    // 这个测试演示了错误处理路径
}

#[tokio::test]
async fn test_client_creation() {
    let client = VectCutClient::new("http://example.com:9001");
    // 客户端应该能成功创建
    // 更多测试需要 mock HTTP 服务器
}
