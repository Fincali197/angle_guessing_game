extern crate sdl2;

use rand::Rng;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::error::Error;
use std::io;
use std::process::exit;

fn how_close(angle: i32, to_guess: f32) -> String {
    let tg = to_guess as i32;
    let diff = tg - angle;
    let abs_diff = diff.abs();

    if abs_diff <= 5 {
        println!("Boiling!");
    } else if abs_diff <= 10 {
        println!("Hot!");
    } else if abs_diff <= 20 {
        println!("Cold...");
    } else {
        println!("Freezing(im literally shaking rn fr)");
    }

    if diff < 0 {
        println!("(down)");
    } else {
        println!("(up)");
    }

    return "".into();
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 600;
    let window_height = 600;

    let window = video_subsystem
        .window("Angler", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut surface = window.into_canvas().build().unwrap();

    const BLACK: Color = Color::RGB(0, 0, 0);
    // const GRAY: Color = Color::RGB(20, 20, 20);
    const WHITE: Color = Color::RGB(255, 255, 255);
    const RED: Color = Color::RGB(225, 108, 83);

    let mut rng = rand::thread_rng();
    let angle_to_guess: f32 = rng.gen_range(20..330) as f32;
    let rotated_angle: f32 = rng.gen_range(0..360) as f32;

    let center_point = Point::new(window_width as i32 / 2, window_height as i32 / 2);
    let line_length = 200;

    let angle_mark_radius = 20;

    surface.set_draw_color(BLACK);
    surface.clear();

    surface.set_draw_color(WHITE);

    for i in 0..angle_to_guess as i32 {
        surface.draw_point(Point::new(
            ((i as f32 + rotated_angle).to_radians().cos() * angle_mark_radius as f32) as i32
                + center_point.x,
            -((i as f32 + rotated_angle).to_radians().sin() * angle_mark_radius as f32) as i32
                + center_point.y,
        ))?;
        surface.draw_point(Point::new(
            ((i as f32 + rotated_angle).to_radians().cos() * angle_mark_radius as f32) as i32
                + center_point.x
                + 1,
            -((i as f32 + rotated_angle).to_radians().sin() * angle_mark_radius as f32) as i32
                + center_point.y,
        ))?;
        surface.draw_point(Point::new(
            ((i as f32 + rotated_angle).to_radians().cos() * angle_mark_radius as f32) as i32
                + center_point.x,
            -((i as f32 + rotated_angle).to_radians().sin() * angle_mark_radius as f32) as i32
                + center_point.y
                + 1,
        ))?;
    }

    surface.thick_line(
        center_point.x as i16,
        center_point.y as i16,
        ((rotated_angle.to_radians().cos() * line_length as f32) as i32 + center_point.x) as i16,
        (-(rotated_angle.to_radians().sin() * line_length as f32) as i32 + center_point.y) as i16,
        2,
        RED,
    )?;
    surface.thick_line(
        center_point.x as i16,
        center_point.y as i16,
        (((angle_to_guess + rotated_angle).to_radians().cos() * line_length as f32) as i32
            + center_point.x) as i16,
        (-((angle_to_guess + rotated_angle).to_radians().sin() * line_length as f32) as i32
            + center_point.y) as i16,
        2,
        RED,
    )?;

    // surface.thick_line(x1, y1, x2, y2, width, color)

    surface.present();
    for i in 0..4 {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).expect("Failed to read line");
        let guessed_number: i32 = input
            .trim()
            .parse()
            .expect("Failed to parse input as integer");
        if guessed_number == angle_to_guess as i32 {
            println!("Yes!! That's it! Thanks for playing!");
            println!("Btw you did it in {} attempt(s), cool ig", i + 1);
            exit(0);
        }
        how_close(guessed_number, angle_to_guess);
        println!("You have {} tries left!", 3 - i);
    }

    how_close(0, angle_to_guess);

    println!("The angle to guess was {}!", angle_to_guess);

    Ok(())
}
