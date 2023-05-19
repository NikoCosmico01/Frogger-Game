use std::any::Any;
use std::cmp::{min, max};

use crate::actor::*;
use crate::rand::*;


pub struct Ball {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32
}
impl Ball {
    pub fn new(pos: Pt) -> Ball {
        Ball{pos: pos, step: pt(4, 4), size: pt(100, 10), speed: 4}
    }
}
impl Actor for Ball {
    fn act(&mut self, arena: &mut ArenaStatus) {
        for other in arena.collisions() {
            if let Some(_) = other.as_any().downcast_ref::<Ghost>() {
            } else {
                let diff = self.pos - other.pos();
                self.step.x = self.speed * if diff.x > 0 { 1 } else { -1 };
                self.step.y = self.speed * if diff.y > 0 { 1 } else { -1 };
            }
        }
        let tl = self.pos + self.step;  // top-left
        let br = tl + self.size - arena.size();  // bottom-right
        if tl.x < 0 { self.step.x = self.speed; }
        if tl.y < 0 { self.step.y = self.speed; }
        if br.x > 0 { self.step.x = -self.speed; }
        if br.y > 0 { self.step.y = -self.speed; }
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { Some(pt(0, 0)) }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}
pub struct Truck {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    left: bool
}
impl Truck{
    pub fn new(pos: Pt, is_left: bool) -> Truck {
        Truck{pos: pos, step: pt(0, 0), size: pt(61,23), speed: 4, left: is_left}
    }
}
impl Actor for Truck {
    fn act(&mut self, arena: &mut ArenaStatus) {
        for other in arena.collisions() {
            if let Some(_) = other.as_any().downcast_ref::<Ghost>() {
            } else {
                let diff = self.pos - other.pos();
                self.step.x = self.speed * if diff.x > 0 { 1 } else { -1 };
            }
        }
       
        if self.left == true { self.step.x = -self.speed; }
        else{
            self.step.x = self.speed;
        }
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { Some(pt(if self.left {192} else { 253 }, 68)) }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Ghost {
    pos: Pt,
    speed: i32,
    visible: bool
}
impl Ghost {
    pub fn new(pos: Pt) -> Ghost {
        Ghost{pos: pos, speed: 4, visible: true}
    }
}
impl Actor for Ghost {
    fn act(&mut self, arena: &mut ArenaStatus) {
        let scr = arena.size();
        let step = pt(randint(-1, 1) * self.speed, randint(-1, 1) * self.speed);
        self.pos = self.pos + step + scr;
        self.pos.x %= scr.x;
        self.pos.y %= scr.y;
        if randint(0, 99) == 0 { self.visible = ! self.visible; }
        if randint(0, 999) == 0 { arena.spawn(Box::new(Ball::new(self.pos))); }
    }
    fn sprite(&self) -> Option<Pt> { Some(pt(20, if self.visible { 0 } else { 20 })) }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { pt(20, 20) }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}


pub struct Frog {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    lives: i32,
    blinking: i32
}
impl Frog {
    pub fn new(pos: Pt) -> Frog {
        Frog{pos: pos, step: pt(0, 0), size: pt(32, 32),
            speed: 32, lives: 3, blinking: 0}
    }
    fn lives(&self) -> i32 { self.lives }
}
impl Actor for Frog {
    fn act(&mut self, arena: &mut ArenaStatus) {
        if self.blinking == 0 {
            for other in arena.collisions() {
                if let Some(_) = other.as_any().downcast_ref::<Truck>() {
                    self.blinking = 60;
                    self.lives -= 1;
                }
            }
        }
        let keys = arena.current_keys();
        self.step = pt(0, 0);
        if keys.contains(&"ArrowUp") {
            self.step.y = -self.speed;
        } else if keys.contains(&"ArrowDown") {
            self.step.y = self.speed;
        }
        if keys.contains(&"ArrowLeft") {
            self.step.x = -self.speed;
        } else if keys.contains(&"ArrowRight") {
            self.step.x = self.speed;
        }
        self.pos = self.pos + self.step;

        let scr = arena.size() - self.size;
        self.pos.x = min(max(self.pos.x, 0), scr.x);  // clamp
        self.pos.y = min(max(self.pos.y, 0), scr.y);  // clamp
        self.blinking = max(self.blinking - 1, 0);
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> {
        if self.blinking > 0 && (self.blinking / 2) % 2 == 0 { None }
        else { Some(pt(256, 256)) }
    }
    fn alive(&self) -> bool { self.lives > 0 }
    fn as_any(&self) -> &dyn Any { self }
}


pub struct BounceGame {
    arena: Arena,
    playtime: i32
}
impl BounceGame {
    fn randpt(size: Pt) -> Pt {
        let mut p = pt(randint(0, size.x), randint(0, size.y));
        while (p.x - size.x / 2).pow(2) + (p.y - size.y / 2).pow(2) < 10000 {
            p = pt(randint(0, size.x), randint(0, size.y));
        }
        return p;
    }
    pub fn new(size: Pt, nballs: i32, nghosts: i32) -> BounceGame {
        let mut arena = Arena::new(size);
        let sizeFrog = pt(size.x/2-16,size.y-3*32);
        arena.spawn(Box::new(Frog::new(sizeFrog)));
        arena.spawn(Box::new(Truck::new(pt(size.x-61,size.y-4*32),true)));
        for _ in 0..nballs {
            arena.spawn(Box::new(Ball::new(BounceGame::randpt(size))));
        }
        for _ in 0..nghosts {
            arena.spawn(Box::new(Ghost::new(BounceGame::randpt(size))));
        }
        BounceGame{arena: arena, playtime: 120}
    }
    pub fn game_over(&self) -> bool { self.remaining_time() <= 0 || self.remaining_lives() <= 0}
    pub fn game_won(&self) -> bool { self.remaining_time() <= 0 }
    pub fn remaining_time(&self) -> i32 {
        self.playtime - self.arena.count() / 30
    }
    pub fn remaining_lives(&self) -> i32 {
        let mut lives = 0;
        let actors = self.actors();
        if let Some(b) = actors.first() {
            if let Some(hero) = b.as_any().downcast_ref::<Frog>() {
                lives = hero.lives();
            }
        }
        lives
    }
    pub fn tick(&mut self, keys: String) { self.arena.tick(keys); }
    pub fn size(&self) -> Pt { self.arena.size() }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { self.arena.actors() }
}
