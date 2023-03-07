use std::f32::consts::PI;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_width: 1 << 8,
        window_height: 1 << 8,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut angle = PI / 2.;
    let position = vec2(screen_width() / 2., screen_height() / 2.);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        angle += 0.05;

        if angle >= 2. * PI {
            angle = 0.;
        }

        clear_background(WHITE);
        let direction = vec2(angle.cos(), angle.sin());
        let end = direction * 100. + position;
        draw_line(position.x, position.y, end.x, end.y, 5., RED);
        next_frame().await
    }
}
