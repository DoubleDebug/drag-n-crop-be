pub mod routes {
    use drag_and_crop::{CropOptions, crop_image};
    use rocket::serde::json::{Json, json, Value};

    #[post("/crop-image", format = "json", data = "<options>")]
    pub fn post_crop_image(options: Json<CropOptions>) -> Value {
        let result = crop_image(&options);
        json!(result)
    }
}
