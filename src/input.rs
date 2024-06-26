use std::io::Write;

use crossterm::{
    cursor::MoveLeft,
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::Print,
};

use crate::{Error, Result};

pub fn wait_for_string<W: Write>(w: &mut W, old_string: &str) -> Result<String> {
    let mut input = old_string.to_string();
    w.write(input.as_bytes())?;
    w.flush()?;

    loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _,
        })) = crossterm::event::read()
        {
            match code {
                KeyCode::Char(c) => {
                    input.push(c);
                    w.write(&[c as u8])?;
                    w.flush()?;
                }
                KeyCode::Enter => {
                    if !input.is_empty() {
                        return Ok(input);
                    }
                }
                KeyCode::Esc => {
                    return Err(Error::Escape);
                }
                KeyCode::Backspace => {
                    if let Some(_) = input.pop() {
                        execute!(w, MoveLeft(1), Print(" "), MoveLeft(1))?;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn wait_for_cmdchar() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _,
        })) = crossterm::event::read()
        {
            match code {
                KeyCode::Char(c) => match c {
                    'a'..='z' => {
                        return Ok(c);
                    }
                    'A'..='Z' => {
                        return Ok(c.to_ascii_lowercase());
                    }
                    _ => {}
                },
                KeyCode::Esc => {
                    return Err(Error::Escape);
                }
                _ => {}
            }
        }
    }
}

pub fn wait_for_u32<W: Write>(w: &mut W, old_u32: Option<u32>) -> Result<u32> {
    let mut input = if let Some(old_u32) = old_u32 {
        old_u32.to_string()
    } else {
        String::new()
    };
    w.write(input.as_bytes())?;
    w.flush()?;
    loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _,
        })) = crossterm::event::read()
        {
            match code {
                KeyCode::Char(c) => match c {
                    '0'..='9' => {
                        input.push(c);
                        w.write(&[c as u8])?;
                        w.flush()?;
                    }
                    _ => {}
                },
                KeyCode::Enter => {
                    let result = input.parse()?;
                    return Ok(result);
                }
                KeyCode::Esc => {
                    return Err(Error::Escape);
                }
                KeyCode::Backspace => {
                    if let Some(_) = input.pop() {
                        execute!(w, MoveLeft(1), Print(" "), MoveLeft(1))?;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn process_event(e: Event) -> Result<Option<char>> {
    let Event::Key(KeyEvent {
        code,
        modifiers: _,
        kind: KeyEventKind::Press,
        state: _,
    }) = e
    else {
        return Ok(None);
    };

    match code {
        KeyCode::Char(c) => match c {
            'a'..='z' => Ok(Some(c)),
            'A'..='Z' => Ok(Some(c.to_ascii_lowercase())),
            '0'..='9' => Ok(Some(c)),
            _ => Ok(None),
        },
        KeyCode::Esc => {
            return Err(Error::Escape);
        }
        _ => Ok(None),
    }
}
