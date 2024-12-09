use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::ServiceConfig;

// 配置 route
pub fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 配置 handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("actix web service is running!")
}

// 实例化http server 并运行
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 构建app，配置route
    let app = move || App::new().configure(general_routes);

    // 运行http server
    HttpServer::new(app).bind("0.0.0.0:3000")?.run().await
}