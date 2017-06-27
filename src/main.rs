extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
mod scanner;
mod gamemap;
mod player;
mod game_info;
mod consts;
use scanner::Scanner;
use player::{PlayerType, ProcHandler};
use game_info::*;
use consts::*;
use std::time;
use std::thread;
use std::process::*;
fn main() {
    let mut players = Vec::new();
    let player_num = if DEBUG {
        let pnum_debug = 4;
        let mut c = Command::new("g++")
            .arg("./player_ai/sample.cc")
            .arg("-std=c++11")
            .spawn()
            .expect("failed to comple");
        let ecode = c.wait().expect("failed to wait on child");
        assert!(ecode.success(), "compile error");
        for i in 0..pnum_debug {
            let p = PlayerType::CommandAI(ProcHandler::new("./a.out"));
            players.push((p, i));
        }
        pnum_debug
    } else {
        // 一応CUIがある
        // 使いづらいのですぐ廃止すると思うけど...
        let mut sc = Scanner::new(std::io::stdin());
        println!("player num: ");
        let player_num: usize = sc.ne();
        assert!(player_num <= 4, "player num <= 4");
        for id in 0..player_num {
            println!("Input player type");
            let s = sc.ne::<String>();
            let p = match &*s {
                "M" => PlayerType::Manual,
                "Z" => PlayerType::ZakoAI,
                "C" => {
                    let command: String = sc.ne();
                    let cmd = ProcHandler::new(&*command);
                    PlayerType::CommandAI(cmd)
                }
                _ => panic!("please write M or Z or C"),
            };
            players.push((p, id));
        }
        player_num
    };
    // ビジュアライザ起動
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(WINDOW_TITLE, [WINDOW_SIZE, WINDOW_SIZE])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut vis = Visualiizer::new(GlGraphics::new(opengl), player_num);
    let event_settings = EventSettings::new().ups(3).max_fps(10);
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            for &mut (ref mut player, id) in &mut players {
                match *player {
                    PlayerType::CommandAI(ref mut cmd) => {
                        let s = vis.game.get_state_str(id);
                        cmd.write(s);
                        let act = cmd.act();
                        vis.game.act(id, act);
                    }
                    _ => {}
                }
                vis.game.update();
            }
            vis.render(&r);
            let millis100 = time::Duration::from_millis(100);
            thread::sleep(millis100);
        }
    }
}
struct Visualiizer {
    gl: GlGraphics,
    game: Game,
    pause: bool,
}

impl Visualiizer {
    fn new(gl: GlGraphics, player_num: usize) -> Visualiizer {
        let mut game = Game::make_random_game(player_num);
        game.update();
        Visualiizer {
            gl: gl,
            game: game,
            pause: false,
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        (&mut self.gl).viewport(0, 0, args.width as i32, args.height as i32);
        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear([1.0, 1.0, 1.0, 1.0], gl)
        });
        self.game.render(&mut self.gl, args);
    }
}
