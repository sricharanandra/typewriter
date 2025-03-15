use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn queue_commands<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_commands(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_commands(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_commands(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_commands(Hide)?;
        Ok(())
    }
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_commands(Show)?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_commands(Print(string))?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        let height = height_u16 as usize;
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
