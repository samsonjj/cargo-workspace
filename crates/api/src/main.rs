use actix_web::{
    get, post,
    web::{self, Redirect},
    App, HttpResponse, HttpServer, Responder,
};

#[get("/api/comments")]
async fn get_comments() -> impl Responder {
    println!("hello");
    let comments = repositories::comments::read_comments().await.unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&comments).unwrap())
}

#[post("/api/comments")]
async fn post_comments(req_body: web::Json<shared::Comment>) -> impl Responder {
    println!("POST");
    repositories::comments::add_comment(req_body.0)
        .await
        .unwrap();
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("listening on port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(Redirect::new("/", "/index.html"))
            .service(get_comments)
            .service(post_comments)
            // must be last, or it will be prioritized over api routes
            .service(actix_files::Files::new("/", "./public/"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
