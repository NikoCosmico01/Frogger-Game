use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use crate::bounce::Frog;
pub mod actor;
pub mod bounce;
pub mod g2d;
pub mod pt2d;
pub mod rand;

pub struct BounceGui {
    game: bounce::BounceGame,
}
impl BounceGui {
    pub fn new() -> BounceGui {
        let game = bounce::BounceGame::new(pt2d::pt(480, 576));
        BounceGui { game }
    }
    pub fn setup(&self) {
        g2d::init_canvas(self.game.size());
        g2d::main_loop(30);
    } //Some(pt(256, 256))

    pub fn draw_time(&self) {
        g2d::draw_text(String::from("Time: "), pt2d::pt(0, 30), 24);
        let height = 16; // Altezza fissa del rettangolo
        let rect_width = 2 * self.game.remaining_time();
        g2d::set_color(0, 143, 57);
        g2d::fill_rect(pt2d::pt(70, 5 + 30), pt2d::pt(rect_width, height));
        g2d::set_color(127, 127, 127);
    }
    pub fn draw_live(&self) {
        g2d::draw_text(String::from("Lives: "), pt2d::pt(0, 4), 24);
        for i in 0..self.game.remaining_lives() {
            g2d::draw_image_clip(
                "frogger.png".to_string(),
                pt2d::pt(70 + i * 35, 7),
                pt2d::pt(68, 6),
                pt2d::pt(23, 17),
            );
        }
    }
    pub fn tick(&mut self) {
        g2d::clear_canvas();

        if self.game.game_over() {
            self.create_background();
            g2d::draw_image_clip(
                "gameOver.png".to_string(),
                pt2d::pt(60, 200),
                pt2d::pt(0, 0),
                pt2d::pt(370, 280),
            );
        } else if self.game.game_won() {
            self.create_background();
            g2d::draw_image_clip(
                "won.png".to_string(),
                pt2d::pt(20, 140),
                pt2d::pt(0, 0),
                pt2d::pt(440, 330),
            );
        } else {
            self.game.tick(g2d::current_keys()); // Game logic
            self.create_background();
            self.draw_time();
            self.draw_live();
            for b in self.game.actors() {
                if let Some(frog) = b.as_any().downcast_ref::<Frog>() {
                    if frog.score()[0] {
                        g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(47,96), pt2d::pt(256, 256), pt2d::pt(32, 32));
                    }
                    if frog.score()[1] {
                        g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(175,96), pt2d::pt(256, 256), pt2d::pt(32, 32));
                    }
                    if frog.score()[2] {
                        g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(271,96), pt2d::pt(256, 256), pt2d::pt(32, 32));
                    }
                    if frog.score()[3] {
                        g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(399,96), pt2d::pt(256, 256), pt2d::pt(32, 32));
                    }
                }
                
                if let Some(img) = b.sprite() {
                    g2d::draw_image_clip("frogger.png".to_string(), b.pos(), img, b.size());
                } else {
                    //g2d::fill_rect(b.pos(), b.size());
                }
            }
        }
    }
    pub fn create_background(&mut self) {
        for i in 0..9 {
            for j in 0..self.game.size().x / 32 + 1 {
                g2d::draw_image_clip(
                    "background.png".to_string(),
                    pt2d::pt(j * 32, 290 + i * (32)),
                    pt2d::pt(32, 0),
                    pt2d::pt(32, 32),
                );
            }
        }
        for i in 0..10 {
            for j in 0..self.game.size().x / 32 + 1 {
                g2d::draw_image_clip(
                    "background.png".to_string(),
                    pt2d::pt(j * 32, 0 + i * (32)),
                    pt2d::pt(0, 0),
                    pt2d::pt(32, 32),
                );
            }
        }
        for j in 0..self.game.size().x / 32 + 4 {
            g2d::draw_image_clip(
                "background.png".to_string(),
                pt2d::pt(j * 29 - 4, 64),
                pt2d::pt(0, 32),
                pt2d::pt(32, 32),
            );
        }
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(-4, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(98, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(126, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(224, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(322, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(350, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        g2d::draw_image_clip(
            "background.png".to_string(),
            pt2d::pt(452, 92),
            pt2d::pt(0, 32),
            pt2d::pt(32, 32),
        );
        for i in 0..2 {
            for j in 0..self.game.size().x / 32 + 1 {
                g2d::draw_image_clip(
                    "frogger.png".to_string(),
                    pt2d::pt(j * 32, 480 + i * (-192)),
                    pt2d::pt(64, 192),
                    pt2d::pt(32, 32),
                );
            }
        }
        g2d::draw_image_clip(
            "frogger.png".to_string(),
            pt2d::pt(128, 544),
            pt2d::pt(0, 255),
            pt2d::pt(224, 32),
        );
    }
}

thread_local! {
    static GUI: RefCell<BounceGui> = RefCell::new(BounceGui::new());
}

#[wasm_bindgen]
pub fn tick() {
    GUI.with(|g| {
        g.borrow_mut().tick();
    });
}

#[wasm_bindgen]
pub fn setup() {
    GUI.with(|g| {
        g.borrow_mut().setup();
    });
}
