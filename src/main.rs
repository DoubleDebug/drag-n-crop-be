#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod web {
  pub mod crop_image;
}

use crate::web::crop_image::routes::post_crop_image;

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![post_crop_image])
}