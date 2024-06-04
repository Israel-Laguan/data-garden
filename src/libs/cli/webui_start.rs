use actix_web::rt::System;

pub fn start_server() {
    System::new("webui-server").block_on(async {
        webui_server::start_server().await;
    });
}
