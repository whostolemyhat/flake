extern crate cairo;

use point::Point;
use generate;
use std::fs::File;
use self::cairo::{ Context, Format, ImageSurface };

const IMAGE_SIZE: i32 = 900;
const SCALE: f64 = 4.0;

fn draw_points(ctx: &Context, points: &Vec<Point>) {
    for point in points {
        let offset = (IMAGE_SIZE as f64 - SCALE) / 2.0;
        let (x, y) = hex_to_cartesian(point.x().into(), point.y().into());
        draw_hexagon(ctx, x + offset, y + offset, SCALE);
    }

    ctx.fill();
}

fn hex_to_cartesian(hex_x: f64, hex_y: f64) -> (f64, f64) {
    (
        SCALE * (3f64.sqrt() * hex_x + 3f64.sqrt() / 2.0 * hex_y),
        SCALE * (3.0 / 2.0 * hex_y)
    )
}

fn draw_hexagon(ctx: &Context, x: f64, y: f64, r: f64) {
    let a = -3f64.sqrt() / 2.0 * r;
    let b = r / 2.0;

    ctx.move_to(a + x, y - b);
    ctx.line_to(x, y - r);
    ctx.line_to(x - a, y - b);
    ctx.line_to(x - a, b + y);
    ctx.line_to(x, r + y);
    ctx.line_to(a + x, b + y);
    ctx.line_to(a + x, y - b);
}

pub fn draw(hash: &str, path: &str) -> Result<(), ::std::io::Error> {
    let flake = generate(&hash);
    let default_output = format!("{}/{}.png", path, hash);
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let ctx = Context::new(&surface);
    draw_points(&ctx, &flake);
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}

