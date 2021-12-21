use conrod::{backend::glium::glium::{self, Surface}};
use std::time::{SystemTime};
use std::{time};
use std::convert::TryFrom;

pub fn open_window()
{
    let mut max_frames = 60;
    let mut max_delta = 1000000000 / max_frames - 100000;


    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    widget_ids!(struct Ids { text });
    let events_loop = glium::glutin::EventsLoop::new();

    let window = glium::glutin::WindowBuilder::new()
        .with_title("Hello Conrod")
        .with_dimensions(WIDTH, HEIGHT);

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let mut current_time = SystemTime::now();
    let mut time_last_frame = current_time;
    '_main: loop {
        // Render the `Ui` and then display it on the screen.
            renderer.fill(&display, ui.draw(), &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();

            // Determine deltaTime
            current_time = SystemTime::now();
            let mut delta_time = current_time
                .duration_since(time_last_frame)
                .expect("Time went backwards, insane")
                .as_nanos();
            
            // Limit FPS by sleeping if higher than 60
            if delta_time < max_delta
            {
                std::thread::sleep(
                    time::Duration::from_nanos(
                    u64::try_from(max_delta - delta_time)
                    .expect("Value too fat for i64)"))
                );
                
                current_time = SystemTime::now();
                delta_time = current_time
                .duration_since(time_last_frame)
                .expect("Time went backwards, insane")
                .as_nanos();
            }

            // Determine FPS and print to console
            let fps = 1000000000 / delta_time;
            println!("{}", fps);

            // Update last frame time
            time_last_frame = current_time;
    }
}