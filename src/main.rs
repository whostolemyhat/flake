extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate snowflake;
extern crate sha2;
#[macro_use]
extern crate tera;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{ middleware, server, App, HttpRequest, Responder, http, HttpResponse, error, Error, fs, Form };
use sha2::{ Sha256, Digest };
use snowflake::{ create_hash };
use snowflake::draw::draw;

// TODO move snowflake crate somewhere
// TODO save in db?
// TODO save as base64?

fn create_flake(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    // TODO salt hash
    let hash = match req.match_info().get("text") {
        Some(text) => {
            let mut hasher = Sha256::default();
            hasher.input(text.as_bytes());
            format!("{:x}", hasher.result())
        },
        None => create_hash(64)
    };

    // check if file exists already - just return it if so
    match draw(&hash, &concat!(env!("CARGO_MANIFEST_DIR"), "/images")) {
        Ok(_) => format!("{:?}", hash),
        Err(e) => format!("Error creating image: {:?}", e)
    };

    let mut ctx = tera::Context::new();
    ctx.add("text", &hash.to_owned());
    let s = req.state().template.render("flake.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn show_form(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let ctx = tera::Context::new();

    let s = req.state().template.render("form.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Deserialize)]
struct Flake {
    name: String
}

// use tuple = multiple handlers (automatic)
fn form_flake(data: (Form<Flake>, HttpRequest<AppState>)) -> Result<HttpResponse, Error> {
    let mut hasher = Sha256::default();
    hasher.input(data.0.name.as_bytes());
    let hash = format!("{:x}", hasher.result());

    // TODO check if file exists already - just return it if so
    match draw(&hash, &concat!(env!("CARGO_MANIFEST_DIR"), "/images")) {
        Ok(_) => format!("{:?}", hash),
        Err(e) => format!("Error creating image: {:?}", e)
    };

    let mut ctx = tera::Context::new();
    ctx.add("text", &hash.to_owned());
    let s = data.1.state().template.render("flake.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// fn index(_req: HttpRequest<AppState>) -> impl Responder {
//     "Hi!"
// }

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
            .handler("/images", fs::StaticFiles::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images")))
            // .resource("/index.html", |r| r.f(|_| "Hello world"))
            // .resource("/flake", |r| r.method(http::Method::GET).f(create_flake))
            .resource("/", |r| {
                r.method(http::Method::GET).f(show_form);
                r.post().with(form_flake);
            })
            .resource("/{text}", |r| r.get().f(create_flake))
            // .resource("/", |r| r.method(http::Method::GET).f(index))
    })
        .bind("127.0.0.1:3099")
        .unwrap()
        .start();

    println!("Server started on 127.0.0.1:3099");
    let _ = sys.run();
}
