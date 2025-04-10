pub mod shape;

use macroquad::prelude::*;
use shape::Shape;

/// 移动速度常量
const MOVEMENT_SPEED: f32 = 400.0;

#[macroquad::main("My First Game")]
async fn main() {
    // 设置随机数种子
    rand::srand(miniquad::date::now() as u64);

    let mut squares = vec![];

    let color = color_u8!(
        rand::gen_range(0, 255),
        rand::gen_range(0, 255),
        rand::gen_range(0, 255),
        rand::gen_range(0, 255)
    );

    let mut circle = Shape {
        size: 64.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color,
    };
    let big_r = circle.size / 2.0;
    let small_r = circle.size / 4.0;

    let mut gameover = false;

    // 加载字体
    let font = match load_ttf_font("my-first-game/assets/fonts/NotoSansSC-Regular.ttf").await {
        Ok(font) => Some(font),
        Err(_) => None,
    };

    let text = "下午好! 刘志成!";
    let text_dimensions = measure_text(text, font.as_ref(), 50, 2.0);

    loop {
        clear_background(DARKBROWN);

        let window_screen_width = screen_width();
        let window_screen_height = screen_height();

        let half_window_width = window_screen_width / 2.0;
        let hafl_window_height = window_screen_height / 2.0;

        if !gameover {
            let delta_time = get_frame_time();
            let move_frame_speed = MOVEMENT_SPEED * delta_time;

            if is_key_down(KeyCode::Right) {
                circle.x += move_frame_speed;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= move_frame_speed;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= move_frame_speed;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += move_frame_speed;
            }

            // 限制移动范围在屏幕内
            circle.x = clamp(circle.x, 0f32, window_screen_width);
            circle.y = clamp(circle.y, 0f32, window_screen_height);

            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(16.0, 64.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: color_u8!(
                        rand::gen_range(0, 255),
                        rand::gen_range(0, 255),
                        rand::gen_range(0, 255),
                        rand::gen_range(0, 255)
                    ),
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            squares.retain(|square| square.y < window_screen_height + square.size);
        }

        if squares
            .iter()
            .any(|square| circle.circle_collides_with(square))
        {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = half_window_width;
            circle.y = hafl_window_height;
            gameover = false;
        }

        draw_circle(circle.x, circle.y, small_r, DARKBLUE);
        draw_circle(circle.x, circle.y, big_r, color);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        if gameover {
            draw_text_ex(
                text,
                half_window_width- text_dimensions.width / 2.0,
                hafl_window_height,
                TextParams {
                    font: font.as_ref(),
                    font_size: 100,
                    color: PINK,
                    ..Default::default()
                },
            );
        }

        next_frame().await;
    }
}
