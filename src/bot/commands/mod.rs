pub mod general;
pub mod music;
use music::{beginloop, endloop, pause, play, queue, resume, skip, stop};
use crate::bot::Command;

pub fn music_commands() -> Vec<Command> {
   vec![
      play(),
      pause(),
      resume(),
      stop(),
      skip(),
      queue(),
      beginloop(),
      endloop(),
   ] 
}