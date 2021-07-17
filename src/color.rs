// use actix_web::web::Data;
// use actix_web::{web, HttpResponse, Responder};
// use async_trait::async_trait;
// use std::sync::Arc;
// use std::time::{SystemTime, UNIX_EPOCH};
// use tracing::debug;
//
// // Configure
// pub fn configure_color(cfg: &mut web::ServiceConfig) {
//     // let bg = Data::new(Blue);
//     let now = SystemTime::now();
//
//     let due = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
//     let c: Box<dyn ColorService> = if due % 2 != 0 {
//         debug!("Green");
//         Box::new(Green {
//             shade: Arc::new(10),
//         })
//     } else {
//         debug!("Blue");
//         Box::new(Blue)
//     };
//
//     let bg = Data::new(c);
//
//     do_routing(bg, cfg)
// }
//
// fn do_routing<T>(data: Data<Box<T>>, cfg: &mut web::ServiceConfig)
// where
//     T: ColorService + ?Sized + 'static,
// {
//     cfg.service(
//         web::resource("/color")
//             .app_data(data.clone())
//             .route(web::get().to(handler::<T>))
//             .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//     );
// }
//
// // Controller
// async fn handler<T>(bg: Data<Box<T>>) -> impl Responder
// where
//     T: ColorService + ?Sized,
// {
//     let color = bg.color().await;
//
//     HttpResponse::Ok().body(color)
// }
//
// #[async_trait]
// pub trait ColorService {
//     async fn color(self: &Self) -> String;
// }
//
// struct Blue;
//
// #[async_trait]
// impl ColorService for Blue {
//     async fn color(self: &Self) -> String {
//         String::from("Blue")
//     }
// }
//
// struct Green {
//     shade: Arc<u8>,
// }
//
// #[async_trait]
// impl ColorService for Green {
//     async fn color(&self) -> String {
//         format!("Green({}%)", *self.shade)
//     }
// }
