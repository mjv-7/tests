/*
By: <Your Name Here>
Date: 2026-06-04
Program Details: <Program Description Here>
*/

mod modules;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::listview::ListView;
use crate::modules::grid::draw_grid;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "tests".to_string(),
        window_width: 1280,
        window_height: 960,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let items = vec!["sprite".to_string(), "Item 2".to_string()];
    let mut list_view = ListView::new(&items, 50.0, 200.0, 100);
    list_view
        .with_colors(BLACK, Some(LIGHTGRAY), Some(SKYBLUE))
        .set_width(250.0)
        .with_padding(10.0);
    let btn_buy = TextButton::new(550.0, 250.0, 100.0, 50.0, "Buy", BLUE, RED, 20);
    let img_bird = StillImage::new(
        "assets/sprite.png",
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    loop {
        clear_background(WHITE);
        list_view.draw();
        if let Some(selected) = list_view.selected_item() {
            match selected.as_str() {
                "Show Cool Bird" => {
                    // Call the draw method built directly into your StillImage struct
                    img_bird.draw();
                }_ => {}
            }
        }
        if btn_buy.click(){
            println!("Buy button clicked!");
        }
        draw_grid(50.0, BLACK);
        next_frame().await;
    }
}
