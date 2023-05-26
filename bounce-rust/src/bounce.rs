use std::any::Any;
use std::cmp::{max, min};

use crate::actor::*;
use crate::rand::*;

#[derive(PartialEq)]
pub enum VehicleType {
    Truck,
    Car1,
    Car2,
}

pub struct Trunk {
    pos: Pt,
    speed: i32,
    step: Pt,
    left: bool,
}
impl Trunk {
    pub fn new(pos: Pt, is_left: bool) -> Trunk {
        Trunk {
            pos: pos,
            speed: 2,
            step: pt(0, 0),
            left: is_left,
        }
    }
}
impl Actor for Trunk {
    fn act(&mut self, arena: &mut ArenaStatus) {
        if self.pos.x + self.size().x <= 0 && self.left == true {
            self.pos.x = arena.size().x;
        }
        if self.pos.x - self.size().x >= arena.size().x && self.left == false {
            self.pos.x = 0 - self.size().x;
        }
        if self.left == true {
            self.step.x = -self.speed;
        } else {
            self.step.x = self.speed;
        }
        self.pos = self.pos + self.step;
    }
    fn sprite(&self) -> Option<Pt> {
        Some(pt(192, 102))
    }
    fn pos(&self) -> Pt {
        self.pos
    }
    fn alive(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn size(&self) -> Pt {
        pt(95, 19)
    }
}

pub struct Turtle {
    pos: Pt,
    speed: i32,
    step: Pt,
    left: bool,
    blinking: i32,
}
impl Turtle {
    pub fn new(pos: Pt, is_left: bool) -> Turtle {
        Turtle {
            pos: pos,
            speed: 2,
            step: pt(0, 0),
            left: is_left,
            blinking: 0,
        }
    }
}
impl Actor for Turtle {
    fn act(&mut self, arena: &mut ArenaStatus) {
        if self.pos.x + self.size().x <= 0 && self.left == true {
            self.pos.x = arena.size().x;
        }
        if self.pos.x - self.size().x >= arena.size().x && self.left == false {
            self.pos.x = 0 - self.size().x;
        }
        if self.left == true {
            self.step.x = -self.speed;
        } else {
            self.step.x = self.speed;
        }
        self.pos = self.pos + self.step;

        //Random blinking
        if randint(0, 1000) == 5 && self.blinking == 0 {
            self.blinking = 10;
        }

        self.blinking = max(self.blinking - 1, 0);
    }
    fn sprite(&self) -> Option<Pt> {
        if self.blinking > 0 && (self.blinking / 2) % 2 == 0 {
            None
        } else {
            Some(pt(224, 132))
        }
    }
    fn pos(&self) -> Pt {
        self.pos
    }
    fn alive(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn size(&self) -> Pt {
        pt(30, 22)
    }
}

pub struct Vehicle {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    left: bool,
    type_veichle: VehicleType,
}
impl Vehicle {
    pub fn new(pos: Pt, is_left: bool, type_v: VehicleType, set_speed: i32) -> Vehicle {
        let size_v;
        if type_v == VehicleType::Truck {
            size_v = pt(61, 23);
        } else if type_v == VehicleType::Car1 {
            size_v = pt(32, 28);
        } else {
            size_v = pt(29, 20)
        }
        Vehicle {
            pos: pos,
            step: pt(0, 0),
            size: size_v,
            speed: set_speed,
            left: is_left,
            type_veichle: type_v,
        }
    }
}
impl Actor for Vehicle {
    fn act(&mut self, arena: &mut ArenaStatus) {
        if self.pos.x + self.size().x <= 0 && self.left == true {
            self.pos.x = arena.size().x;
        }
        if self.pos.x - self.size().x >= arena.size().x && self.left == false {
            self.pos.x = 0 - self.size().x;
        }
        if self.left == true {
            self.step.x = -self.speed;
        } else {
            self.step.x = self.speed;
        }
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt {
        self.pos
    }
    fn size(&self) -> Pt {
        self.size
    }
    fn sprite(&self) -> Option<Pt> {
        Some(pt(
            if self.type_veichle == VehicleType::Truck {
                if self.left {
                    192
                } else {
                    253
                }
            } else if self.type_veichle == VehicleType::Car1 {
                192
            } else {
                256
            },
            if self.type_veichle == VehicleType::Truck {
                68
            } else if self.type_veichle == VehicleType::Car1 {
                if self.left {
                    34
                } else {
                    2
                }
            } else {
                if self.left {
                    6
                } else {
                    38
                }
            },
        ))
    }

    fn alive(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Frog {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    lives: i32,
    is_game_won: bool,
    blinking: i32,
}
impl Frog {
    pub fn new(pos: Pt) -> Frog {
        Frog {
            pos: pos,
            step: pt(0, 0),
            size: pt(32, 32),
            speed: 32,
            lives: 3,
            is_game_won: false,
            blinking: 0,
        }
    }
    fn lives(&self) -> i32 {
        self.lives
    }
    fn game_won(&self) -> bool {
        self.is_game_won
    }
}
impl Actor for Frog {
    fn act(&mut self, arena: &mut ArenaStatus) {
        self.step = pt(0, 0);
        let mut collide_with_trunk = false;
        let mut collide_with_turtle = false;
        if (self.pos.x >= 0 && self.pos.x <= 32
            || self.pos.x >= 96 && self.pos.x <= 160
            || self.pos.x >= 224 && self.pos.x <= 256
            || self.pos.x >= 320 && self.pos.x <= 384
            || self.pos.x >= 448 && self.pos.x <= 480)
            && self.pos.y >= 96
            && self.pos.y < 128
        {
            self.is_game_won = true;
        }
        if self.blinking == 0 {
            for other in arena.collisions() {
                if other.as_any().downcast_ref::<Vehicle>().is_some() {
                    self.lives -= 1;
                    if self.lives > 0 {
                        self.pos.x = 223;
                        self.pos.y = 480;
                        self.blinking = 20;
                    }
                }
                if let Some(trunk) = other.as_any().downcast_ref::<Trunk>() {
                    self.step.x = trunk.step.x;
                    collide_with_trunk = true;
                }
                if let Some(turtle) = other.as_any().downcast_ref::<Turtle>() {
                    self.step.x = turtle.step.x;
                    collide_with_turtle = true;
                }
            }
            if collide_with_trunk == false
                && collide_with_turtle == false
                && self.pos.y < arena.size().y - 10 * 32 + 13
                && self.pos.y > arena.size().y - 16 * 32 + 13
            {
                self.lives -= 1;
                if self.lives > 0 {
                    self.pos.x = 223;
                    self.pos.y = 288;
                    self.blinking = 20;
                }
            }
        }

        let keys = arena.current_keys();
        if keys.contains(&"ArrowUp") == true
            && keys.contains(&"ArrowUp") != arena.previous_keys().contains(&"ArrowUp")
            && self.blinking == 0
        {
            self.step.y = -self.speed;
        } else if keys.contains(&"ArrowDown") == true
            && keys.contains(&"ArrowDown") != arena.previous_keys().contains(&"ArrowDown")
            && self.pos.y < 500
        {
            self.step.y = self.speed;
        }
        if keys.contains(&"ArrowLeft") == true
            && keys.contains(&"ArrowLeft") != arena.previous_keys().contains(&"ArrowLeft")
        {
            self.step.x = -self.speed;
        } else if keys.contains(&"ArrowRight") == true
            && keys.contains(&"ArrowRight") != arena.previous_keys().contains(&"ArrowRight")
        {
            self.step.x = self.speed;
        }

        self.pos = self.pos + self.step;
        let scr = arena.size() - self.size;
        self.pos.x = min(max(self.pos.x, 0), scr.x); // clamp
        self.pos.y = min(max(self.pos.y, 0), scr.y); // clamp

        self.blinking = max(self.blinking - 1, 0);
    }
    fn pos(&self) -> Pt {
        self.pos
    }
    fn size(&self) -> Pt {
        self.size
    }
    fn sprite(&self) -> Option<Pt> {
        if self.blinking > 0 && (self.blinking / 2) % 2 == 0 {
            None
        } else {
            if self.lives > 0 {
                Some(pt(256, 256))
            } else {
                Some(pt(2, 192))
            }
        }
    }
    fn alive(&self) -> bool {
        self.lives > 0
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct BounceGame {
    arena: Arena,
    playtime: i32,
}
impl BounceGame {
    pub fn new(size: Pt) -> BounceGame {
        let mut arena = Arena::new(size);
        arena.spawn(Box::new(Turtle::new(
            pt(size.x - 61, size.y - 320 + 6),
            true,
        )));
        arena.spawn(Box::new(Turtle::new(
            pt(size.x - 87, size.y - 320 + 6),
            true,
        )));
        arena.spawn(Box::new(Turtle::new(
            pt(size.x - 113, size.y - 320 + 6),
            true,
        )));
        let size_frog = pt(size.x / 2 - 16, size.y - 3 * 32);
        for i in 1..5 {
            let random_number = randint(20, 1000);
            if i % 2 == 0 {
                arena.spawn(Box::new(Trunk::new(
                    pt(
                        size.x - 61 - (i * random_number),
                        size.y - (10 + i) * 32 + 13,
                    ),
                    true,
                )));
            } else {
                arena.spawn(Box::new(Trunk::new(
                    pt(
                        size.x - 61 - (i * random_number),
                        size.y - (10 + i) * 32 + 13,
                    ),
                    false,
                )));
            }
            println!("{}", random_number);
        }
        arena.spawn(Box::new(Frog::new(size_frog)));
        arena.spawn(Box::new(Vehicle::new(
            pt(size.x - 61, size.y - 4 * 32),
            false,
            VehicleType::Car1,
            4,
        )));
        arena.spawn(Box::new(Vehicle::new(
            pt(size.x - 61, size.y - 6 * 32),
            true,
            VehicleType::Car2,
            5,
        )));
        arena.spawn(Box::new(Vehicle::new(
            pt(size.x - 30, size.y - 5 * 32),
            true,
            VehicleType::Truck,
            6,
        )));
        arena.spawn(Box::new(Vehicle::new(
            pt(size.x - 30, size.y - 7 * 32),
            true,
            VehicleType::Car1,
            4,
        )));
        arena.spawn(Box::new(Vehicle::new(
            pt(size.x - 30, size.y - 8 * 32),
            false,
            VehicleType::Truck,
            6,
        )));

        BounceGame {
            arena: arena,
            playtime: 120,
        }
    }
    pub fn game_over(&self) -> bool {
        self.remaining_time() <= 0 || self.remaining_lives() <= 0
    }
    pub fn game_won(&self) -> bool {
        self.winning_game()
    }
    pub fn remaining_time(&self) -> i32 {
        self.playtime - self.arena.count() / 30
    }
    pub fn remaining_lives(&self) -> i32 {
        let mut lives = 0;
        let actors = self.actors();
        for actor in actors {
            if let Some(hero) = actor.as_any().downcast_ref::<Frog>() {
                lives = hero.lives();
            }
        }
        lives
    }
    pub fn winning_game(&self) -> bool {
        let mut win_game = false;
        let actors = self.actors();
        for actor in actors {
            if let Some(hero) = actor.as_any().downcast_ref::<Frog>() {
                win_game = hero.game_won();
            }
        }
        win_game
    }
    pub fn tick(&mut self, keys: String) {
        self.arena.tick(keys);
    }
    pub fn size(&self) -> Pt {
        self.arena.size()
    }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> {
        self.arena.actors()
    }
}
