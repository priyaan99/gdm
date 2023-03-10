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
    let vec_a = Vec2::from_angle(PI).normalize();
    let mut vec_b = Vec2::from_angle(PI / 2.).normalize();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let center = vec2(screen_width() / 2., screen_height() / 2.);

        vec_b = (vec2(mouse_position().0, mouse_position().1) - center).normalize();

        clear_background(WHITE);

        const LINE_LEN: f32 = 25.;

        draw_text(
            &format!("Dot Product {}", vec_a.dot(vec_b)),
            10.,
            30.,
            24.,
            RED,
        );

        draw_line(
            center.x,
            center.y,
            center.x + vec_a.x * LINE_LEN,
            center.y + vec_a.y * LINE_LEN,
            5.,
            RED,
        );
        draw_line(
            center.x,
            center.y,
            center.x + vec_b.x * LINE_LEN,
            center.y + vec_b.y * LINE_LEN,
            5.,
            RED,
        );
        next_frame().await
    }
}
