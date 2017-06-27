use piston::input::*;
use opengl_graphics::GlGraphics;
use std::collections::BTreeMap;
use gamemap::{Field, Coord};
use consts::*;
use rand::{self, Rng};
use std::cmp::*;
const MOVES: [Coord; 5] = [Coord { x: 0, y: -1 },
                           Coord { x: -1, y: 0 },
                           Coord { x: 1, y: 0 },
                           Coord { x: 0, y: 1 },
                           Coord { x: 0, y: 0 }];

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
    player_bom: usize,
    explosion_list: Vec<(Coord, i32, i32)>,
}

fn cd_ok(cd: Coord, s: usize) -> bool {
    cd.x >= 0 && cd.y >= 0 && cd.x < s as i32 && cd.y < s as i32
}
impl Game {
    pub fn make_random_game(player_num: usize) -> Game {
        let mut rng = rand::thread_rng();
        let size: usize = rng.gen_range(FIELD_SIZE_MIN, FIELD_SIZE_MAX);
        let mut player = Vec::new();
        for i in 0..player_num {
            let x = if i & 1 == 0 { 0 } else { size - 1 };
            let y = if i <= 1 { 0 } else { size - 1 };
            let cd = Coord::new(x as i32, y as i32);
            player.push(Player::new(cd, 0));
        }
        let oil_num: usize = rng.gen_range(size, size * 2);
        let bom_num: usize = rng.gen_range(size / 4, size / 3);
        Game {
            size: size,
            player_num: player_num,
            player: player,
            oil_list: BTreeMap::new(),
            bom_list: BTreeMap::new(),
            field: Field::new(FieldState::None, size, size),
            oil_num: oil_num,
            bom_num: bom_num,
            player_bom: 0,
            explosion_list: Vec::new(),
        }
    }
    pub fn act(&mut self, player_id: usize, ac: Action) {
        let cur = self.player[player_id].cd;
        match ac {
            Action::Move(id) => {
                if id < 4 {
                    let nxt = cur + MOVES[id];
                    if !cd_ok(nxt, self.size) {
                        if DEBUG {
                            println!("Action wa Rejcected: Move, id: {}", player_id);
                        }
                        return;
                    }
                    self.player[player_id].cd = nxt;
                    if let FieldState::Oil(g) = self.field[nxt] {
                        self.player[player_id].galon += g;
                        self.field[nxt] = FieldState::None;
                        self.oil_list.remove(&nxt);
                    }
                }
            }
            Action::PickBom => {
                let mut flag = false;
                if let FieldState::Bom(b) = self.field[cur] {
                    if let BomCat::Safe(d) = b {
                        self.player_bom += 1;
                        self.player[player_id].bom = Some(d);
                        self.bom_list.remove(&cur);
                        self.field[cur] = FieldState::None;
                        flag = true;
                    }
                }
                if !flag && DEBUG {
                    println!("Action was Rejected: Pickbom, id: {}", player_id);
                }
            }
            Action::DropBom => {
                let mut flag = false;
                if self.field[cur] == FieldState::None {
                    if let Some(d) = self.player[player_id].bom {
                        self.player_bom -= 1;
                        self.player[player_id].bom = None;
                        let bom = BomCat::Ready((d, player_id));
                        self.bom_list.insert(cur, bom);
                        self.field[cur] = FieldState::Bom(bom);
                        flag = true;
                    }
                }
                if !flag && DEBUG {
                    println!("Action was Rejected: Dropbom, id: {}", player_id);
                }
            }
        }
    }
    pub fn update(&mut self) -> bool {
        // 起爆判定 高速化は後でいいや...
        let mut res = false;
        for i in 0..self.player_num {
            let ref mut p = self.player[i];
            let cur = p.cd;
            for dx in -5..6 {
                for dy in -5..6 {
                    let nxt = cur + Coord::new(dx, dy);
                    let d = max(dx.abs(), dy.abs());
                    if !self.field.id_ok(&nxt) {
                        continue;
                    }
                    if let FieldState::Bom(b) = self.field[nxt] {
                        if let BomCat::Ready((dist, id)) = b {
                            if id == i || dist < d {
                                continue;
                            }
                            self.field[nxt] = FieldState::None;
                            self.bom_list.remove(&nxt);
                            self.explosion_list.push((nxt, dist, 0));
                            res = true;
                        }
                    }
                }
            }
        }

        // 爆弾、石油の補充
        let mut rng = rand::thread_rng();
        while self.oil_num > self.oil_list.len() {
            let x = (rng.gen::<usize>() % self.size) as i32;
            let y = (rng.gen::<usize>() % self.size) as i32;
            let cd = Coord::new(x, y);
            if self.field[cd] != FieldState::None {
                continue;
            }
            let g = rng.gen_range(MIN_OIL, MAX_OIL);
            self.oil_list.insert(cd, g);
            self.field[cd] = FieldState::Oil(g);
        }
        for ref mut p in &mut self.player {
            if let FieldState::Oil(g) = self.field[p.cd] {
                p.galon += g;
                self.field[p.cd] = FieldState::None;
                self.oil_list.remove(&p.cd);
            }
        }
        while self.bom_num > self.bom_list.len() + self.player_bom {
            let x = (rng.gen::<usize>() % self.size) as i32;
            let y = (rng.gen::<usize>() % self.size) as i32;
            let cd = Coord::new(x, y);
            if self.field[cd] != FieldState::None {
                continue;
            }
            let d = rng.gen_range(MIN_BOM, MAX_BOM);
            let bom = BomCat::Safe(d);
            self.bom_list.insert(cd, bom);
            self.field[cd] = FieldState::Bom(bom);
        }
        res
    }

    pub fn get_state_str(&self, player_id: usize) -> String {
        let mut s = format!("{}\n", self.size);
        s += &*format!("{} {}\n", self.player_num, player_id);
        for p in &self.player {
            s += &*format!("{} {} {}\n", p.cd.x, p.cd.y, p.galon);
        }
        s += &*format!("{}\n", self.oil_list.len());
        for (cd, galon) in &self.oil_list {
            s += &*format!("{} {} {}\n", cd.x, cd.y, galon);
        }
        s += &*format!("{}\n", self.bom_list.len());
        for (cd, bom) in &self.bom_list {
            let owner = match *bom {
                BomCat::Safe(_) => -1,
                BomCat::Ready((_, id)) => id as i32,
            };
            s += &*format!("{} {} {}\n", cd.x, cd.y, owner);
        }
        s
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) -> bool {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        use graphics::grid::Grid;
        use graphics::line::Line;
        use graphics::rectangle;
        use graphics::Transformed;
        use graphics::polygon;
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
        let fb = bsize as f64;
        let triangle: [[f64; 2]; 3] = [[0.5 * fb, 0.0], [0.0, fb], [fb, fb]];

        for y in 0..self.size {
            for x in 0..self.size {
                let fx = (x * bsize) as f64;
                let fy = (y * bsize) as f64;
                match self.field[(x, y)] {
                    FieldState::Oil(galon) => {
                        // とりあえず色の濃さだけ変えてみる
                        let col_level = 0.5 + 0.5 * galon as f32 / 100.0;
                        let color: [f32; 4] = [0.0, 0.0, 0.0, col_level];
                        gl.draw(args.viewport(), |c, gl| {
                            rectangle(color, block, c.transform.trans(fx, fy), gl);
                        });
                    }
                    FieldState::Bom(bom) => {
                        let color = match bom {
                            BomCat::Ready((_, id)) => USER_COLOR[id],
                            BomCat::Safe(_) => BLUE,
                        };
                        gl.draw(args.viewport(), |c, gl| {
                            polygon(color, &triangle, c.transform.trans(fx, fy), gl);
                        });
                    }
                    _ => {}
                }
            }
        }
        let offset = bsize as f64 / 2.0;
        let block = rectangle::square(0.0, 0.0, bsize as f64 / 2.0);
        for i in 0..self.player_num {
            if self.player[i].is_alive == false {
                continue;
            }
            let fx = (self.player[i].cd.x * bsize as i32) as f64 +
                     if i & 1 == 1 { offset } else { 0.0 };
            let fy = (self.player[i].cd.y * bsize as i32) as f64 + if i > 1 { offset } else { 0.0 };
            gl.draw(args.viewport(),
                    |c, gl| { rectangle(USER_COLOR[i], block, c.transform.trans(fx, fy), gl); });
        }
        if self.explosion_list.is_empty() {
            return false;
        }

        let block = rectangle::square(0.0, 0.0, bsize as f64);
        let mut explist_nxt = Vec::new();
        for &(cd, dist, turn) in &self.explosion_list {
            for dx in -turn..turn + 1 {
                for dy in -turn..turn + 1 {
                    let cur = cd + Coord::new(dx, dy);
                    let fx = (cur.x * bsize as i32) as f64;
                    let fy = (cur.y * bsize as i32) as f64;
                    if self.field.id_ok(&cur) {
                        self.field[cur] = match self.field[cur] {
                            FieldState::Oil(_) => {
                                self.oil_list.remove(&cur);
                                FieldState::None
                            }
                            _ => self.field[cur],
                        };
                        for ref mut p in &mut self.player {
                            if p.cd == cur {
                                p.is_alive = false;
                                println!("@_@ Death, {:?}", p.cd);
                            }
                        }
                        gl.draw(args.viewport(),
                                |c, gl| { rectangle(RED, block, c.transform.trans(fx, fy), gl); });
                    }
                }
            }
            if dist > turn {
                explist_nxt.push((cd, dist, turn + 1));
            }
        }
        self.explosion_list = explist_nxt;
        self.explosion_list.is_empty()
    }
}



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Move(usize),
    PickBom,
    DropBom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BomCat {
    Safe(i32), // 安全(爆破距離)
    Ready((i32, usize)), // 起爆待機(爆破距離、プレイヤーID)
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
    is_alive: bool,
}

impl Player {
    fn new(cd: Coord, g: i32) -> Player {
        Player {
            cd: cd,
            galon: g,
            bom: None,
            is_alive: true,
        }
    }
}
