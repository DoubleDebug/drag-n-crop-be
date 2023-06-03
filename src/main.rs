#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
pub mod web {
  pub mod crop_image;
  pub mod crop_video;
  pub mod catchers;
  pub mod firebase;
}

use crate::web::crop_image::routes::post_crop_image;
use crate::web::crop_video::routes::post_crop_video;
use crate::web::catchers::{ default_catcher, unprocessable_entity };

#[launch]
fn rocket() -> _ {
  rocket
    ::build()
    .mount("/", routes![post_crop_image, post_crop_video])
    .register("/", catchers![default_catcher, unprocessable_entity])
}
