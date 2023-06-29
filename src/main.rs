#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
pub mod web {
  pub mod endpoints;
  pub mod catchers;
  pub mod firebase;
}
pub mod utils {
  pub mod init;
  pub mod cors;
}
use utils::init::init;
use utils::cors::CORS;
use rocket::fs::{ FileServer, Options };
use crate::web::endpoints::routes::{
  post_crop_image,
  post_crop_video,
  options_crop_image,
  options_crop_video,
};
use crate::web::catchers::{ default_catcher, unprocessable_entity };

#[launch]
fn rocket() -> _ {
  init();

  rocket
    ::build()
    .attach(CORS)
    .mount("/", FileServer::new("./static/swagger-ui", Options::Index).rank(1))
    .mount("/docs", FileServer::new("./docs", Options::Index).rank(2))
    .mount(
      "/api",
      routes![post_crop_image, post_crop_video, options_crop_image, options_crop_video]
    )
    .register("/api", catchers![default_catcher, unprocessable_entity])
}
