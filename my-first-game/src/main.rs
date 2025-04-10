mod shape;

use std::ops::Add;

use macroquad::prelude::*;
use shape::Shape;

/// 移动速度常量
const MOVEMENT_SPEED: f32 = 400.0;

#[macroquad::main("My First Game")]
async fn main() {
    // 设置随机数种子
    rand::srand(miniquad::date::now() as u64); 

    let window_screen_width = screen_width();
    let window_screen_height = screen_height();

    // 使用 color_u8! 宏创建颜色
    let color = color_u8!(1, 55, 99, 99);
    println!("{:?}", color);

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: window_screen_width / 2.0,
        y: window_screen_height / 2.0,
        color,
    };

    loop {
        clear_background(DARKBROWN);

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

        draw_circle(circle.x.add(1f32), circle.y.add(1f32), 32.0, color);
        draw_circle(circle.x, circle.y, 16.0, BLACK);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        next_frame().await;
    }
}
