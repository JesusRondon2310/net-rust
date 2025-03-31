use rocket::response::content::RawJson;
use rust_code::helpers::database::{fetch_data, postgres_connection};

#[macro_use]
extern crate rocket;

#[get("/")]
async fn fetching() -> RawJson<String> {
    let pool = postgres_connection()
        .await
        .expect("error when connecting to database");
    let expose = fetch_data(&pool).await.expect("error to fetching data");
    return RawJson(
        serde_json::to_string(&expose)
            .unwrap()
            .trim_end()
            .to_string(),
    );
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetching])
}
