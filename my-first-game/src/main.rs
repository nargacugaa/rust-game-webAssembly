use macroquad::prelude::*;

#[macroquad::main("My First Game")]
async fn main() {
    loop {
        clear_background(DARKGREEN);
        next_frame().await;
    }
}
