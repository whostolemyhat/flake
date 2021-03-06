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

use actix_web::{ middleware, server, App, HttpRequest, http, HttpResponse, error, Error, fs, Form };
use sha2::{ Sha256, Digest };
use snowflake::{ create_hash };
use snowflake::draw::draw;

// TODO move snowflake crate somewhere
// TODO save in db?
// TODO save as base64?

fn create_flake(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    // TODO salt hash
    let mut text_str = String::new();
    let hash = match req.match_info().get("text") {
        Some(text) => {
            text_str = text.to_string();
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
    ctx.add("text", &text_str);
    ctx.add("hash", &hash.to_owned());
    let s = req.state().template.render("flake.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn show_form(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let ctx = tera::Context::new();

    let s = req.state().template.render("flake.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
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
    ctx.add("text", &data.0.name);
    ctx.add("hash", &hash.to_owned());
    let s = data.1.state().template.render("flake.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn list_images(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let folder = concat!(env!("CARGO_MANIFEST_DIR"), "/images");
    let mut images: Vec<String> = vec![];

    // TODO don't read stuff out of the dir
    for image in ::std::fs::read_dir(folder).expect("Can't read folder") {
        let image = image.expect("error");
        match image.path().file_name() {
            Some(name) => images.push(name.to_str().unwrap().to_string()),
            None => ()
        };
    }

    // get all images in folder
    let mut ctx = tera::Context::new();
    ctx.add("images", &images);
    let s = req.state().template.render("all-images.html", &ctx).map_err(|_| error::ErrorInternalServerError("Template error")).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
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
            .handler("/images", fs::StaticFiles::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images")))
            .handler("/css", fs::StaticFiles::new(concat!(env!("CARGO_MANIFEST_DIR"), "/public/css")))
            .resource("/", |r| {
                r.method(http::Method::GET).f(show_form);
                r.post().with(form_flake);
            })
            .resource("/all", |r| r.get().f(list_images))
            .resource("/{text}", |r| r.get().f(create_flake))
    })
        .bind("0.0.0.0:3099")
        .unwrap()
        .start();

    println!("Server started on 0.0.0.0:3099");
    let _ = sys.run();
}
