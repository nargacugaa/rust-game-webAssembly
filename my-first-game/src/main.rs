pub mod shape;

use macroquad::prelude::*;
use shape::Shape;

/// 移动速度常量
const MOVEMENT_SPEED: f32 = 400.0;

#[macroquad::main("My First Game")]
async fn main() {
    // 设置随机数种子
    rand::srand(miniquad::date::now() as u64);

    // 方块容器
    let mut squares = vec![];
    // 子弹容器
    let mut bullets: Vec<Shape> = vec![];

    let color = color_u8!(
        rand::gen_range(0, 255),
        rand::gen_range(0, 255),
        rand::gen_range(0, 255),
        rand::gen_range(0, 255)
    );

    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color,
        collided: false,
    };
    let big_r = circle.size / 2.0;
    let small_r = circle.size / 4.0;

    let mut gameover = false;

    // 加载字体
    let font = match load_ttf_font("my-first-game/assets/fonts/NotoSansSC-Regular.ttf").await {
        Ok(font) => Some(font),
        Err(_) => None,
    };

    let text = "Game Over! Press Space to Restart";
    // font_scale: 缩放倍数
    let text_dimensions = measure_text(text, font.as_ref(), 30, 1.0);

    loop {
        clear_background(BLANK);

        let window_screen_width = screen_width();
        let window_screen_height = screen_height();

        let half_window_width = window_screen_width / 2.0;
        let hafl_window_height = window_screen_height / 2.0;

        if !gameover {
            let mut delta_time = get_frame_time();
            // 慢动作调试
            if is_key_down(KeyCode::A) {
                delta_time *= 0.3;
            }

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

            // 同屏子弹4
            if is_key_pressed(KeyCode::Space) && bullets.len() < 4 {
                let size = rand::gen_range(10.0, 25.0);
                let color = color_u8!(
                    rand::gen_range(0, 255),
                    rand::gen_range(0, 255),
                    rand::gen_range(0, 255),
                    rand::gen_range(0, 255)
                );

                bullets.push(Shape {
                    size,
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.5,
                    color,
                    collided: false,
                });
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
                    collided: false,
                });
            }

            // 方块移动
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            // 子弹射击
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }

            // 移除超出屏幕的方块
            squares.retain(|square| square.y < window_screen_height + square.size);
            // 移除超出屏幕的子弹
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            // 移除被子弹击中的方块
            squares.retain(|square| !square.collided);
            // 移除击中方块的子弹
            bullets.retain(|bullet| !bullet.collided);
        }

        // 判断circle与方块的碰撞
        if squares
            .iter()
            .any(|square| circle.circle_collides_with(square))
        {
            gameover = true;
        }
        // 判断子弹与方块的碰撞
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.circle_collides_with(square) {
                    square.collided = true;
                    bullet.collided = true;
                }
            }
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = half_window_width;
            circle.y = hafl_window_height;
            gameover = false;
        }

        // 渲染子弹
        for bullet in &bullets {
            // if rand::gen_range(0, 99) > 50 {
            //     draw_circle_lines(bullet.x, bullet.y, bullet.size / 2.0, 5.0, bullet.color);
            // } else {
            //     draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
            // }
            draw_circle_lines(bullet.x, bullet.y, bullet.size / 2.0, 5.0, bullet.color);
        }

        // 渲染circle
        draw_circle(circle.x, circle.y, small_r, DARKBLUE);
        draw_circle(circle.x, circle.y, big_r, color);

        // 渲染方块
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
                half_window_width - text_dimensions.width / 2.0,
                hafl_window_height,
                TextParams {
                    font: font.as_ref(),
                    font_size: 30,
                    color: PINK,
                    ..Default::default()
                },
            );
        }

        next_frame().await;
    }
}
