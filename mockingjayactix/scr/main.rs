/*An actix microservice that has 3 routes"
A. / that returns a hello world
B. / that uses the return_string function to return whatever you feed it
C. /that returns "ok bye" when you feed it "Stop" */
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

//create a function that returns hello world
#[get("/")]
async fn hello() -> impl Responder {
    println!("Hello world");
    HttpResponse::Ok().body("Hello world!")
}
//create a function that returns ok bye when you feed it stop
#[get("/stop")]
async fn stop() -> impl Responder {
    println!("ok bye");
    HttpResponse::Ok().body("ok bye")
}
//create a function that returns whatever you feed it
#[get("/mockingjay/{mock}")]
async fn return_same(mock: web::Path<String>) -> impl Responder {
    let s = &mock;
    println!("{}", s);
    HttpResponse::Ok().body(mockingjayactix::return_string(s.to_string()))
}
//create a main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to show that the server is running
    println!("Server running at http://localhost:8080/");
    HttpServer::new(|| App::new().service(hello).service(stop).service(return_same))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
