extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate snowflake;
#[macro_use]
extern crate tera;

use actix_web::{ middleware, server, App, HttpRequest, Responder, http, HttpResponse, error, Error, fs };

use snowflake::{ create_hash };
use snowflake::draw::draw;

fn create_flake(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let hash = match req.match_info().get("text") {
        Some(text) => String::from(text),
        None => create_hash(64)
    };

    // check if file exists already

    match draw(&hash) {
        Ok(_) => format!("{:?}", hash),
        Err(e) => format!("Error creating image: {:?}", e)
    };

    let mut ctx = tera::Context::new();
    ctx.add("text", &hash.to_owned());
    let s = req.state().template.render("flake.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


fn index(_req: HttpRequest<AppState>) -> impl Responder {
    "Hi!"
}

struct AppState {
    template: tera::Tera
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello");

    server::new(|| {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::with_state(AppState{ template: tera })
            .middleware(middleware::Logger::default())
            .handler("/images", fs::StaticFiles::new("images"))
            .resource("/index.html", |r| r.f(|_| "Hello world"))
            // .resource("/flake", |r| r.method(http::Method::GET).f(create_flake))
            .resource("/flake/{text}", |r| r.method(http::Method::GET).f(create_flake))
            .resource("/", |r| r.method(http::Method::GET).f(index))
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Server started on 127.0.0.1:8080");
    let _ = sys.run();
}
