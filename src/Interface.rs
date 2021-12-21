use conrod::{backend::glium::glium::{self, Surface}, glium::glutin::os::unix::x11::ffi::Time};
use std::time::{SystemTime, UNIX_EPOCH};

fn open_window()
{
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    widget_ids!(struct Ids { text });
    let ids = Ids::new(ui.widget_id_generator());
    let mut events_loop = glium::glutin::EventsLoop::new();
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
    'main: loop {
        // Render the `Ui` and then display it on the screen.
        // if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, ui.draw(), &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 1.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();


            current_time = SystemTime::now();
            let time_diff = current_time
                .duration_since(time_last_frame)
                .expect("Time went backwards lol")
                .as_nanos();
            let fps = 1000000000 / time_diff;
            println!("{}", fps);
            time_last_frame = current_time;
        // }
    }
}