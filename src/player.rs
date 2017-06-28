use std::error::Error;
use scanner::Scanner;
use std::io::prelude::*;
use game_info::Action;

pub enum PlayerType {
    ZakoAI,
    CommandAI(ProcHandler),
    Manual,
}

fn get_action<R: Read>(resource: R) -> Action {
    let mut sc = Scanner::new(resource);
    let act: usize = match sc.next() {
        Some(n) => n,
        None => panic!("たぶんAIがセグフォしてる"),
    };
    match act {
        val if val == 1 => {
            let moveid: usize = sc.ne();
            Action::Move(moveid)
        }
        val if val == 2 => Action::PickBom,
        val if val == 3 => Action::DropBom,
        _ => Action::Move(4),
    }
}

// Command AI用
use std::process::*;
pub struct ProcHandler {
    my_proc: Child,
}
impl ProcHandler {
    pub fn new(cmdstr: &str) -> ProcHandler {
        let my_proc = match Command::new(cmdstr)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Ok(p) => p,
            Err(why) => panic!("couldn't exec command : {}", Error::description(&why)),
        };
        ProcHandler { my_proc }
    }
    pub fn act(&mut self) -> Action {
        let stdout = self.my_proc.stdout.as_mut().unwrap();
        get_action(stdout)
    }
    pub fn write(&mut self, s: String) {
        let mut stdin = self.my_proc.stdin.as_mut().unwrap();
        let _ = stdin.write(s.as_bytes());
    }
}

// 一応明示的にKillしておく
impl Drop for ProcHandler {
    fn drop(&mut self) {
        match self.my_proc.kill() {
            Ok(_) => println!("Killed Process"),
            Err(_) => println!("SIGKILL failed"),
        }
    }
}
