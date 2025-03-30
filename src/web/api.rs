use crate::ui_interface; // 通过 crate 根引用
use warp::Filter;
use std::net::SocketAddr;
use hbb_common::log;

 

pub async fn start_web_server(port: u16) {
    log::info!("Starting web server on port {}", port);
    
    let connect_route = warp::path!("connect" / String)
        .and(warp::addr::remote())
        .map(move |id: String, addr: Option<SocketAddr>| {
            if let Some(addr) = addr {
                if !addr.ip().is_loopback() {
                    return warp::reply::json(&serde_json::json!({"error": "local only"}));
                }
            }
            
            ui_interface::new_remote(  // 使用完整路径调用
                id.clone(),
                "default".to_string(),
                false
            );
            
            warp::reply::json(&serde_json::json!({
                "status": "connecting",
                "id": id
            }))
        });

    // 服务状态检查路由
    let status_route = warp::path!("status")
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    // 合并路由
    let routes = connect_route.or(status_route);

    // 启动服务器
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}