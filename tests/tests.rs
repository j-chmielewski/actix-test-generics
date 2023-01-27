use actix_web::{App, test::init_service};


// What is the correct return type?
async fn init_app() -> () {
    let app = App::new();
    init_service(app).await
}

#[actix_web::test]
async fn test() {
    let app = init_app().await;
}
