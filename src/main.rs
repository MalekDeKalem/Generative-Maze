
use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin, Seedable };

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 750;

pub struct LinePos {
    pub start: Point2,
    pub end: Point2,
}



fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _lines: Vec<LinePos>,
    _rand_r: u8,
    _rand_g: u8,
    _rand_b: u8,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Template")
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

        let mut _lines: Vec<LinePos> = Vec::new();

        for j in (-HEIGHT as i32 / 2..HEIGHT as i32 / 2).step_by(25) {
            for i in (-WIDTH as i32  / 2..WIDTH as i32 / 2).step_by(25) {
                let rand_num = random_range(0.0, 1.0);
                if rand_num < 0.5 {
                    let start = pt2(i as f32, j as f32 - 25 as f32);
                    let end = pt2(i as f32 + 25 as f32, j as f32);
                    _lines.push(LinePos { start: (start), end: (end) });
                } else {
                    let start = pt2(i as f32, j as f32);
                    let end = pt2(i as f32 + 25 as f32, j as f32 - 25 as f32);
                    _lines.push(LinePos { start: (start), end: (end) });
                }
            }
        }
        let _rand_r = random_range(0, 255) as u8;
        let _rand_g: u8 = random_range(0, 255) as u8;
        let _rand_b: u8 = random_range(0, 255) as u8;
    Model { _lines,  _rand_r, _rand_g, _rand_b } 
}

fn update(app: &App, _model: &mut Model, update: Update) {
    let perlin_r = Perlin::new();
    let perlin_g = Perlin::new();
    let perlin_b = Perlin::new();
    perlin_r.set_seed(46823);
    perlin_g.set_seed(56173);
    perlin_b.set_seed(34527);
    let time_coord_r = [app.time as f64 * 0.5, 0.0];
    let time_coord_g = [app.time as f64 * 0.1, 0.0];
    let time_coord_b = [app.time as f64 * 0.2, 0.0];

    let noise_r = perlin_r.get(time_coord_r) as f32;
    let noise_g = perlin_g.get(time_coord_g) as f32;
    let noise_b = perlin_b.get(time_coord_b) as f32;
    let min_v = 0.0;
    let max_v = 255.0;
    _model._rand_r = map_range(noise_r, -1.0, 1.0, min_v, max_v) as u8;
    _model._rand_g = map_range(noise_g * noise_r, -1.0, 1.0, min_v, max_v) as u8;
    _model._rand_b = map_range(noise_b * noise_b, -1.0, 1.0, min_v, max_v) as u8;
}


fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    for (index, line) in _model._lines.iter().enumerate() {

        let start = line.start;
        let end = line.end;

        draw.line()
            .start(start)
            .end(end)
            .stroke_weight(2.0)
            .color(Srgb::<u8>::new(_model._rand_r, _model._rand_g, _model._rand_b));
    }
    
    draw.to_frame(app, &frame).unwrap();
}





