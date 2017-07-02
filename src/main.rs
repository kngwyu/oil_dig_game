extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
#[macro_use]
extern crate conrod;
use piston::window::{WindowSettings, AdvancedWindow};
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
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
    let mut child = thread::spawn(move || {make_menu()});
    let mut players = Vec::new();
    let player_num = if DEBUG {
        let pnum_debug = 4;
        // compile
        let ai_name = ["sample", "bom_ai", "killer_ai"];
        for s in ai_name.iter() {
            let fname = format!("{}{}{}", "./player_ai/", s, ".cc");
            let mut ai = Command::new("g++")
                .arg(fname)
                .arg("-std=c++11")
                .arg("-o")
                .arg(s)
                .spawn()
                .expect("failed to comple");
            let ecode = ai.wait().expect("failed to wait on child");
            assert!(ecode.success(), "compile error");
        }
        for i in 0..pnum_debug {
            let ai = format!("{}{}", "./", ai_name[i % 3]);
            let p = PlayerType::CommandAI(ProcHandler::new(&ai));
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
    let mut window: GlutinWindow = WindowSettings::new(WINDOW_TITLE, [WINDOW_SIZE, WINDOW_SIZE])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut vis = Visualiizer::new(GlGraphics::new(opengl), player_num);
    let event_settings = EventSettings::new().ups(3).max_fps(10);
    let mut events = Events::new(event_settings);
    let wait_time = time::Duration::from_millis(200);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            if vis.pause {
                window.set_title(format!("{}(pause)", WINDOW_TITLE));
                thread::sleep(wait_time);
                continue;
            }
            window.set_title(format!("{} turn: {}", WINDOW_TITLE, vis.game.turn));
            let mut end_cnt = 0;
            for &mut (ref mut player, id) in &mut players {
                if vis.explosing {
                    break;
                }
                match *player {
                    PlayerType::CommandAI(ref mut cmd) => {
                        let s = vis.game.get_state_str(id);
                        cmd.write(s);
                        let act = cmd.act();
                        end_cnt += vis.game.act(id, act);
                    }
                    _ => {}
                }
                vis.explosing |= vis.game.update();
            }
            if end_cnt as usize == player_num {
                break;
            }
            vis.explosing &= vis.render(&r);
            thread::sleep(wait_time);
        }
        // if let Some(p) = e.release_args() {
        //     vis.release(&p);
        // }
    }
    for p in &vis.game.player {
        println!("{}", p.galon);
    }
    child.join();
}

struct Visualiizer {
    gl: GlGraphics,
    game: Game,
    pause: bool,
    explosing: bool,
}

impl Visualiizer {
    fn new(gl: GlGraphics, player_num: usize) -> Visualiizer {
        let mut game = Game::make_random_game(player_num);
        game.update();
        Visualiizer {
            gl: gl,
            game: game,
            pause: false,
            explosing: false,
        }
    }
    fn render(&mut self, args: &RenderArgs) -> bool {
        (&mut self.gl).viewport(0, 0, args.width as i32, args.height as i32);
        self.gl.draw(args.viewport(), |_, gl| {
            graphics::clear([1.0, 1.0, 1.0, 1.0], gl)
        });
        self.game.render(&mut self.gl, args)
    }
    fn release(&mut self, button: &Button) {
        match *button {
            Button::Keyboard(key) => {
                match key {
                    Key::P => self.pause = if self.pause { false } else { true },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


widget_ids!{
    struct Ids {canvas, list}
}

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
fn make_menu() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 300;

    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("PlotPath Demo")
        .with_multisampling(4)
        .build_glium()
        .unwrap();
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let ids = Ids::new(ui.widget_id_generator());
    ui.fonts.insert_from_file(FONT_PATH).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut event_loop = MyEventLoop::new();
    let mut list = vec![true; 16];
        'main: loop {

            // Handle all events.
            for event in event_loop.next(&display) {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed =>
                        break 'main,
                    _ => {},
                }
            }

            set_ui(ui.set_widgets(), &mut list, &ids);

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
}

fn set_ui(ref mut ui: conrod::UiCell, list: &mut [bool], ids: &Ids) {
    use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(conrod::color::DARK_CHARCOAL)
        .set(ids.canvas, ui);

    let (mut items, scrollbar) = widget::List::flow_down(list.len())
        .item_size(50.0)
        .scrollbar_on_top()
        .middle_of(ids.canvas)
        .wh_of(ids.canvas)
        .set(ids.list, ui);

    while let Some(item) = items.next(ui) {
        let i = item.i;
        let label = format!("item {}: {}", i, list[i]);
        let toggle = widget::Toggle::new(list[i])
            .label(&label)
            .label_color(conrod::color::WHITE)
            .color(conrod::color::LIGHT_BLUE);
        for v in item.set(toggle, ui) {
            list[i] = v;
        }
    }

    if let Some(s) = scrollbar {
        s.set(ui)
    }
}

pub struct MyEventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl MyEventLoop {

    pub fn new() -> Self {
        MyEventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(display.poll_events());

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}
