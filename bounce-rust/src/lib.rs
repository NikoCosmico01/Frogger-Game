use wasm_bindgen::prelude::*;
use std::cell::RefCell;

pub mod actor;
pub mod bounce;
pub mod g2d;
pub mod pt2d;
pub mod rand;

pub struct BounceGui {
    game: bounce::BounceGame
}
impl BounceGui {
    pub fn new() -> BounceGui {
        let game = bounce::BounceGame::new(pt2d::pt(480, 576), 0, 0);
        BounceGui{game}
    }
    pub fn setup(&self) {
        g2d::init_canvas(self.game.size());
        g2d::main_loop(30);
    }
    pub fn tick(&mut self) {
        g2d::clear_canvas();
		self.createBackground();
        for b in self.game.actors() {
            if let Some(img) = b.sprite() {
                g2d::draw_image_clip("frogger.png".to_string(), b.pos(), img, b.size());
            } else {
                //g2d::fill_rect(b.pos(), b.size());
            }
        }
        let txt = format!("Lives: {} Time: {}",
            self.game.remaining_lives(), self.game.remaining_time());
        g2d::draw_text(txt, pt2d::pt(0, 0), 24);

        if self.game.game_over() {
            g2d::alert("Game over".to_string());
            g2d::close_canvas();
        } else if self.game.game_won() {
            g2d::alert("Game won".to_string());
            g2d::close_canvas();
        } else {
            self.game.tick(g2d::current_keys());  // Game logic
        }
    }
	pub fn createBackground(&mut self){
		
		for i in 0..9{
			for j in 0..self.game.size().x/32{
				g2d::draw_image_clip("background.png".to_string(), pt2d::pt(j*32, 290+i*(32)), pt2d::pt(32, 0), pt2d::pt(32, 32));
			}
		}
		for i in 0..10{
			for j in 0..self.game.size().x/32{
				g2d::draw_image_clip("background.png".to_string(), pt2d::pt(j*32, 0+i*(32)), pt2d::pt(0, 0), pt2d::pt(32, 32));
			}
		}
		for j in 0..self.game.size().x/32+1{
			g2d::draw_image_clip("background.png".to_string(), pt2d::pt(j*30, 64), pt2d::pt(0, 32), pt2d::pt(32, 32));
		}
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(0, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(96, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(128, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(224, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(320, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(352, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
		g2d::draw_image_clip("background.png".to_string(), pt2d::pt(448, 92), pt2d::pt(0, 32), pt2d::pt(32, 32));
        for i in 0..2{
			for j in 0..self.game.size().x/32{
				g2d::draw_image_clip("frogger.png".to_string(), pt2d::pt(j*32, 480+i*(-192)), pt2d::pt(64, 192), pt2d::pt(32, 32));
			}
		}
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
