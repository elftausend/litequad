use litequad::prelude::*;

async fn run() {
    loop {
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await;
    }
}

fn main() {
    let conf = Conf {
        window_width: 395,
        window_height: 395,
        ..Default::default()
    };
    litequad::Window::from_config(conf, run());
}
