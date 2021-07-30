use crate::direction::Direction;

pub enum Command {
    Quit,
    Jump(Direction),
}