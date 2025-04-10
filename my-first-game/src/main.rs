use std::ops::Add;

use macroquad::prelude::*;

#[macroquad::main("My First Game")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 400.0;

    let window_screen_width = screen_width();
    let window_screen_height = screen_height();

    let mut x = window_screen_width / 2.0;
    let mut y = window_screen_height / 2.0;

    // 使用 color_u8! 宏创建颜色
    let color = color_u8!(1, 55, 99, 99);
    println!("{:?}", color);

    loop {
        clear_background(DARKBROWN);

        let delta_time = get_frame_time();
        let move_frame_speed = MOVEMENT_SPEED * delta_time;

        if is_key_down(KeyCode::Right) {
            x += move_frame_speed;
        }
        if is_key_down(KeyCode::Left) {
            x -= move_frame_speed;
        }
        if is_key_down(KeyCode::Up) {
            y -= move_frame_speed;
        }
        if is_key_down(KeyCode::Down) {
            y += move_frame_speed;
        }

        draw_circle(x.add(1f32), y.add(1f32), 32.0, color);
        draw_circle(x, y, 16.0, BLACK);

        x = clamp(x, 0f32, window_screen_width);
        y = clamp(y, 0f32, window_screen_height);

        next_frame().await;
    }
}
