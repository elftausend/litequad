use litequad::prelude::{clear_background, draw_circle, next_frame, screen_width, screen_height, YELLOW, WHITE, Conf, draw_poly_angle, GREEN};

async fn run() {
    loop {
        clear_background(WHITE);

        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_poly_angle(screen_width() / 2., screen_height() / 2., 20, 40., 120., GREEN);

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
