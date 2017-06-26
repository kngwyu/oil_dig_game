use piston::input::*;
use opengl_graphics::GlGraphics;
use std::collections::BTreeMap;
use gamemap::{Field, Coord};
use consts::*;
use rand::{self, Rng};

const MOVES: [Coord; 5] = [
    Coord { x: 0, y: -1 },
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: 0, y: 0 },
];

// 爆弾の実装はまた今度...
#[derive(Debug)]
pub struct Game {
    pub size: usize,
    player_num: usize,
    player: Vec<Player>,
    oil_list: BTreeMap<Coord, i32>,
    bom_list: BTreeMap<Coord, BomCat>,
    field: Field<FieldState>,
    oil_num: usize,
    bom_num: usize,
}

fn cd_ok(cd: Coord, s: usize) -> bool {
    cd.x >= 0 && cd.y >= 0 && cd.x < s as i32 && cd.y < s as i32
}
impl Game {
    pub fn make_random_game(player_num: usize) -> Game {
        let mut rng = rand::thread_rng();
        let size = 20 + rng.gen::<usize>() % 20;
        let mut player = Vec::new();
        for i in 0..player_num {
            let x = if i & 1 == 0 { 0 } else { size - 1 };
            let y = if i <= 1 { 0 } else { size - 1 };
            let cd = Coord::new(x as i32, y as i32);
            player.push(Player::new(cd, 0, None));
        }
        let oil_num = size + rng.gen::<usize>() as usize % size * 2;
        let bom_num = 0;
        Game {
            size: size,
            player_num: player_num,
            player: player,
            oil_list: BTreeMap::new(),
            bom_list: BTreeMap::new(),
            field: Field::new(FieldState::None, size, size),
            oil_num: oil_num,
            bom_num: bom_num,
        }
    }
    pub fn act(&mut self, player_id: usize, ac: Action) {
        println!("@_@{:?} {}", ac, player_id);
        match ac {
            Action::Move(id) => {
                if id < 4 {
                    let nxt = self.player[player_id].cd + MOVES[id];
                    if !cd_ok(nxt, self.size) {
                        println!("@_@ {:?} {}", nxt, self.size);
                        return;
                    }
                    println!("@_@ OK");
                    self.player[player_id].cd = nxt;
                    if let FieldState::Oil(_) = self.field[nxt] {
                        self.player[player_id].galon += self.oil_list[&nxt];
                        println!("{:?}", self.oil_list.remove(&nxt));
                        self.field[nxt] = FieldState::None;
                    }
                }
            }
            _ => {} // todo: 爆弾
        }
    }
    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        while self.oil_num > self.oil_list.len() {
            let x = (rng.gen::<usize>() % self.size) as i32;
            let y = (rng.gen::<usize>() % self.size) as i32;
            let g = rng.gen::<u32>() as i32 % 100;
            let cd = Coord::new(x, y);
            self.oil_list.insert(cd, g);
            self.field[cd] = FieldState::Oil(g);
        }
        while self.bom_num > self.bom_list.len() {}
    }

    pub fn get_state_str(&self, player_id: usize) -> String {
        let mut s = format!("{}\n", self.size);
        s += &*format!("{}\n", self.oil_list.len());
        println!("{:?}", self.oil_list.contains_key(&Coord::new(4, 5)));
        for (cd, galon) in &self.oil_list {
            s += &*format!("{} {} {}\n", cd.x, cd.y, galon);
        }
        s += &*format!("{}\n", self.bom_list.len());
        for (cd, bom) in &self.bom_list {}
        s += &*format!("{} {}\n", self.player_num, player_id);
        for p in &self.player {
            s += &*format!("{} {} {}\n", p.cd.x, p.cd.y, p.galon);
        }
        s
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        use graphics::grid::Grid;
        use graphics::line::Line;
        use graphics::rectangle;
        use graphics::Transformed;
        let bsize = WINDOW_SIZE as usize / self.size;
        gl.draw(args.viewport(), |c, gl| {
            Grid {
                cols: self.size as u32,
                rows: self.size as u32,
                units: bsize as f64,
            }
            .draw(&Line::new(BLACK, 0.8), &c.draw_state, c.transform, gl);
        });
        let block = rectangle::square(0.0, 0.0, bsize as f64);
        for y in 0..self.size {
            for x in 0..self.size {
                let color = match self.field[(x, y)] {
                    FieldState::Oil(_) => BLACK,
                    FieldState::Bom(_) => RED,
                    _ => continue,
                };
                let fx = (x * bsize) as f64;
                let fy = (y * bsize) as f64;
                gl.draw(args.viewport(), |c, gl| {
                    rectangle(color, block, c.transform.trans(fx, fy), gl);
                });
            }
        }
        let offset = bsize as f64 / 4.0;
        let block = rectangle::square(0.0, 0.0, bsize as f64 / 2.0);
        for i in 0..self.player_num {
            let fx = (self.player[i].cd.x * bsize as i32) as f64 + offset;
            let fy = (self.player[i].cd.y * bsize as i32) as f64 + offset;
            gl.draw(args.viewport(), |c, gl| {
                rectangle(BLUE, block, c.transform.trans(fx, fy), gl);
            });
        }
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Nop,
    Move(usize),
    PickBom,
    DropBom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BomCat {
    Safe,
    Ready(i32),
    Wait(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FieldState {
    Bom(BomCat),
    Oil(i32),
    None,
}

#[derive(Clone, Debug)]
struct Player {
    cd: Coord,
    galon: i32,
    bom: Option<i32>,
}

impl Player {
    fn new(cd: Coord, g: i32, b: Option<i32>) -> Player {
        Player {
            cd: cd,
            galon: g,
            bom: b,
        }
    }
}
