use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use std::{fs, path::Path};
use mime_guess::from_path;
use minijinja::{context, Environment, Value};
use eyre::Result;

fn render_template(path: &str, ctx: Value) -> Result<String> {
    // get template string
    let full_path = format!("./src/templates/{}", path);
    let temp_str = fs::read_to_string(full_path)?;

    // assemble template
    let mut env = Environment::new();
    env.add_template("main", &temp_str)?;
    let template = env.get_template("main")?;

    // render the result
    Ok(template.render(&ctx)?)
}

#[get("/rs/{resource}")]
async fn resource(resource: web::Path<String>) -> impl Responder {
    let file_path = format!("./src/resources/{}", resource.into_inner());

    // check if file exists in resources folder, otherwise return 404
    if Path::new(&file_path).exists() {
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                let mime_type = from_path(&file_path).first_or_octet_stream();
                HttpResponse::Ok().append_header((header::CONTENT_TYPE, mime_type.as_ref())).body(contents)
            },
            Err(err) => HttpResponse::InternalServerError().body(format!("Error reading file: {}", err)),
        }
    } else {
        HttpResponse::NotFound().body("File not found")
    }
}

#[get("/")]
async fn home() -> impl Responder {
    // template vars
    let path = "base.html.jinja";
    let ctx = context! { message => "<h1>Hello World!</h1>" };

    // return rendered template
    match render_template(path, ctx) {
        Ok(res) => HttpResponse::Ok().body(res),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error rendering template: {}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(resource)
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
