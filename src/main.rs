use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ManualRequest {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ManualResponse {
    message: String,
}

async fn index() -> impl Responder {
    format!("Hello world!")
}

async fn manual(item: web::Json<ManualRequest>, _req: HttpRequest) -> HttpResponse {
    let message = format!("Hello {}!", item.name);
    HttpResponse::Ok().json(ManualResponse { message })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/manual").route(web::post().to(manual)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let endpoint = "/";
        let app = test::init_service(
            App::new().service(web::resource(endpoint).route(web::get().to(index))),
        )
        .await;

        let req = test::TestRequest::get().uri(endpoint).to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);

        let res_body = match res.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Failed unwrap response body."),
        };

        assert_eq!(res_body, "Hello world!");

        Ok(())
    }

    #[actix_rt::test]
    async fn test_manual() -> Result<(), Error> {
        let endpoint = "/manual";
        let app = test::init_service(
            App::new().service(web::resource(endpoint).route(web::post().to(manual))),
        )
        .await;

        let req = test::TestRequest::post()
            .uri(endpoint)
            .set_json(&ManualRequest {
                name: "anonymous".to_owned(),
            })
            .to_request();
        let res = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);

        let res_body = match res.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Failed unwrap response body."),
        };

        assert_eq!(res_body, r##"{"message":"Hello anonymous!"}"##);

        Ok(())
    }
}
