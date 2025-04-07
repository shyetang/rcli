use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

// HTTP服务状态结构体，用于存储服务的根路径
#[derive(Debug)]
struct HttpServeState {
    // 服务的根路径
    path: PathBuf,
}

// 处理HTTP服务的主函数
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    // 创建监听地址,绑定所有网卡的指定端口
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    // 记录服务启动信息
    info!("Serving {:?} on {}", path, addr);
    // 创建服务状态实例
    let state = HttpServeState { path: path.clone() };
    /*let dir_service = ServeDir::new(path)
    .append_index_html_on_directories(true)
    .precompressed_gzip()
    .precompressed_br()
    .precompressed_deflate()
    .precompressed_zstd();*/
    let router = Router::new()
        // 配置路由处理所有路径
        .nest_service("/tower", ServeDir::new(path))
        .route("/{*path}", get(file_handler))
        // 添加共享状态
        .with_state(Arc::new(state));
    // 创建axum路由器

    // 创建TCP监听器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    // 启动HTTP服务
    axum::serve(listener, router).await?;
    // 返回成功
    Ok(())
}

// 处理文件请求的处理函数
async fn file_handler(
    // 获取共享状态
    State(state): State<Arc<HttpServeState>>,
    // 获取请求路径
    Path(path): Path<String>,
) -> (StatusCode, String) {
    // 构建完整的文件路径
    let p = std::path::Path::new(&state.path).join(path);
    // 记录读取文件的日志
    info!("Reading file {:?}", p);
    // 检查文件是否存在
    if !p.exists() {
        // 文件不存在时返回404
        (
            StatusCode::NOT_FOUND,
            format!("File {:?} not found", p.display()),
        )
    } else {
        // TODO: test p is a directory
        // if it is a directory, return a list of files/subdirectories
        // as <li> <a href="/path/to/file">file</a><li>
        // <html><body><ul>...</ul></body></html>
        // 尝试读取文件内容
        match tokio::fs::read_to_string(p).await {
            // 读取成功
            Ok(content) => {
                // 记录读取字节数
                info!("Read {} bytes", content.len());
                // 返回文件内容
                (StatusCode::OK, content)
            }
            // 读取失败
            Err(e) => {
                // 记录错误信息
                warn!("Error reading file: {:?}", e);
                // 返回500错误
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error reading file: {:?}", e),
                )
            }
        }
    }
}
