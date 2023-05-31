#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
pub mod web {
  pub mod crop_image;
  pub mod catchers;
}
use crate::web::crop_image::routes::post_crop_image;
use crate::web::catchers::{ default_catcher, unprocessable_entity };

#[launch]
fn rocket() -> _ {
  rocket
    ::build()
    .mount("/", routes![post_crop_image])
    .register("/", catchers![default_catcher, unprocessable_entity])
}
