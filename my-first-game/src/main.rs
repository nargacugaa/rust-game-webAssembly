mod shader;
pub mod shape;
mod state;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;

use macroquad::prelude::*;

use shader::{FRAGMENT_SHADER, VERTEX_SHADER};
use shape::Shape;
use state::GameState;

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

    // 加载字体
    let font = match load_ttf_font("my-first-game/assets/fonts/NotoSansSC-Regular.ttf").await {
        Ok(font) => Some(font),
        Err(_) => None,
    };

    // 当前分数
    let mut score: u32 = 0;
    // 历史最高分
    let mut high_score: u32 = load_high_score();

    // 设置游戏状态
    let mut game_state = GameState::MainMenu;
    // 判断碰撞是否发生
    let mut collides = false;
    // 添加时间缩放变量
    let mut time_scale = 1.0;

    #[cfg(not(target_arch = "wasm32"))]
    let mut direction_modifier: f32 = 0.0;
    #[cfg(not(target_arch = "wasm32"))]
    let render_target = render_target(320, 150);
    #[cfg(not(target_arch = "wasm32"))]
    render_target.texture.set_filter(FilterMode::Nearest);
    #[cfg(not(target_arch = "wasm32"))]
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLANK);
        #[cfg(not(target_arch = "wasm32"))]
        {
            material.set_uniform("iResolution", (screen_width(), screen_height()));
            material.set_uniform("direction_modifier", direction_modifier);
            gl_use_material(&material);
            draw_texture_ex(
                &render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            gl_use_default_material();
        }

        let window_screen_width = screen_width();
        let window_screen_height = screen_height();

        let half_window_width = window_screen_width / 2.0;
        let hafl_window_height = window_screen_height / 2.0;

        // 获取当前帧的时间
        let mut delta_time = get_frame_time();
        delta_time *= time_scale;

        // 慢动作调试
        if is_key_down(KeyCode::A) {
            delta_time *= 0.3;
        }

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                    squares.clear();
                    bullets.clear();
                    circle.x = half_window_width;
                    circle.y = hafl_window_height;
                    collides = false;
                    time_scale = 1.0;
                    score = 0;
                }

                let text = "PRESS SPACE TO START!";
                let text_dimensions = measure_text(text, font.as_ref(), 50, 1.0);
                draw_text_ex(
                    text,
                    half_window_width - text_dimensions.width / 2.0,
                    hafl_window_height,
                    TextParams {
                        font: font.as_ref(),
                        font_size: 30,
                        color: RED,
                        ..Default::default()
                    },
                );
            }
            GameState::Playing => {
                // Pause
                if is_key_down(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                let move_frame_speed = MOVEMENT_SPEED * delta_time;

                // 生成方块的逻辑
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
                for square in squares.iter_mut().filter(|square| !square.collided) {
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

                // 移除击中方块的子弹
                bullets.retain(|bullet| !bullet.collided);

                if !collides {
                    if is_key_down(KeyCode::Right) {
                        circle.x += move_frame_speed;
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            direction_modifier += 0.05 * delta_time;
                        }
                    }
                    if is_key_down(KeyCode::Left) {
                        circle.x -= move_frame_speed;
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            direction_modifier += 0.05 * delta_time;
                        }
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

                    // 移除被子弹击中的方块
                    squares.retain(|square| !square.collided);
                }

                // 判断circle与方块的碰撞
                if squares
                    .iter_mut()
                    .any(|square| match circle.circle_collides_with(square) {
                        true => {
                            collides = true;
                            square.collided = true;
                            true
                        }
                        false => false,
                    })
                {
                    if time_scale == 1.0 {
                        time_scale = 0.3;
                    }

                    // 逐渐恢复正常速度
                    if time_scale < 1.0 {
                        time_scale += 0.01;
                        if time_scale > 1.0 {
                            time_scale = 1.0;
                            // 保存最高分
                            if score == high_score {
                                save_high_score(score);
                            }

                            game_state = GameState::GameOver;
                        }
                    }
                }
                // 判断子弹与方块的碰撞
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.circle_collides_with(square) {
                            square.collided = true;
                            bullet.collided = true;

                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                        }
                    }
                }

                // 渲染子弹
                for bullet in &bullets {
                    // region: 这会放bullet每帧在圆圈和实心圆之间跳动
                    // if rand::gen_range(0, 99) > 50 {
                    //     draw_circle_lines(bullet.x, bullet.y, bullet.size / 2.0, 5.0, bullet.color);
                    // } else {
                    //     draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
                    // }
                    // endregion
                    draw_circle_lines(bullet.x, bullet.y, bullet.size / 2.0, 5.0, bullet.color);
                }

                // 渲染circle
                draw_circle(circle.x, circle.y, small_r, DARKBLUE);
                draw_circle(circle.x, circle.y, big_r, color);
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }

                let text = "PAUSED! PRESS SPACE TO CONTINUE";
                let text_dimensions = measure_text(text, font.as_ref(), 30, 1.0);
                // 暂停游戏
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
            GameState::GameOver => {
                // 游戏结束重开
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MainMenu;
                }

                let text = "Game Over! Press Space to Restart";
                // font_scale: 缩放倍数
                let text_dimensions = measure_text(text, font.as_ref(), 30, 1.0);
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
                if score == high_score {
                    let congratulation_text = "Congratulations!";
                    draw_text_ex(
                        congratulation_text,
                        half_window_width - text_dimensions.width / 3.0,
                        hafl_window_height + text_dimensions.height + 10.0,
                        TextParams {
                            font: font.as_ref(),
                            font_size: 30,
                            color: PINK,
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // 在暂停时也渲染方块和分数
        match game_state {
            GameState::Paused | GameState::Playing => {
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

                // region: 渲染分数和历史最高分数
                // 渲染分数
                draw_text_ex(
                    format!("当前分数: {}", score).as_str(),
                    10.0,
                    35.0,
                    TextParams {
                        font: font.as_ref(),
                        font_size: 25,
                        color: WHITE,
                        ..Default::default()
                    },
                );

                // 渲染历史最高分数
                let highscore_text = format!("历史最高分: {}", high_score);
                let highscore_text_dimensions =
                    measure_text(highscore_text.as_str(), font.as_ref(), 25, 1.0);
                draw_text_ex(
                    highscore_text.as_str(),
                    screen_width() - highscore_text_dimensions.width - 10.0,
                    35.0,
                    TextParams {
                        font: font.as_ref(),
                        font_size: 25,
                        color: YELLOW,
                        ..Default::default()
                    },
                );
                // endregion
            }
            _ => {}
        }

        next_frame().await;
    }
}

/// 读取高度分数
///
/// 根据当前不同的平台有不同的实现
fn load_high_score() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        0
        // #[wasm_bindgen]
        // extern "C" {
        //     #[wasm_bindgen(js_namespace = window)]
        //     fn loadHighScore() -> u32;
        // }

        // loadHighScore()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        fs::read_to_string("my-first-game/assets/data/highscore.dat")
            .map_or(Ok(0), |s| s.parse::<u32>())
            .unwrap_or(0)
    }
}

/// 保存最高的分数
fn save_high_score(score: u32) {
    #[cfg(target_arch = "wasm32")]
    {
        // #[wasm_bindgen]
        // extern "C" {
        //     #[wasm_bindgen(js_namespace = window)]
        //     fn saveHighScore(score: u32);
        // }

        // saveHighScore(score);
        println!("saveHighScore: {}", score);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        fs::write("my-first-game/assets/data/highscore.dat", score.to_string()).ok();
    }
}
