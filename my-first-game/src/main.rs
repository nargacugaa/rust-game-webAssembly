use macroquad::prelude::*;

#[macroquad::main("My First Game")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(DARKBROWN);

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.5;
        }
        if is_key_down(KeyCode::Up) {
            y -= 2.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }

        // draw_circle(x, y, 16.0, color_u8!(1.00, 0.43, 0.76, 1.00));
        draw_circle(x, y, 16.0, BLACK);

        next_frame().await;
    }
}
