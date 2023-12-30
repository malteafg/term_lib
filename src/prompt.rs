use std::fmt::Display;
use std::io::Write;

use crossterm::{cursor, queue, style::Print};

use crate::command::Command;
use crate::display::*;
use crate::input;
use crate::{Error, Result};

pub fn input_u32<W: Write>(w: &mut W, text: &str) -> Result<u32> {
    iter(w, text.split("\n"))?;
    line(w, "(Input should be a number)")?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    input::wait_for_u32(w)
}

pub fn edit_string<W: Write>(w: &mut W, text: &str, old_string: &str) -> Result<String> {
    iter(w, text.split("\n"))?;
    queue!(w, cursor::Show)?;
    w.flush()?;

    let result = input::wait_for_string(w, old_string)?.trim().to_string();

    queue!(w, cursor::Hide)?;
    Ok(result)
}

pub fn input_string<W: Write>(w: &mut W, text: &str) -> Result<String> {
    edit_string(w, text, "")
}

pub fn confirmation<W: Write>(w: &mut W, text: &str) -> Result<bool> {
    iter(w, text.split("\n"))?;
    line(w, "(y)es or (n)o?")?;
    w.flush()?;

    loop {
        match input::wait_for_cmdchar()? {
            'y' => return Ok(true),
            'n' => return Ok(false),
            _ => {}
        }
    }
}

pub fn select_from_list<W: Write, D: Display, I: Iterator<Item = D> + Clone>(
    w: &mut W,
    text: Option<&str>,
    options: I,
) -> Result<D> {
    let mut cmds: Vec<char> = Vec::new();
    ('a'..='z').for_each(|l| cmds.push(l));
    ('A'..='Z').for_each(|l| cmds.push(l));

    let mut num_iter = 0usize;
    let mut selected = '\n';
    for (i, item) in options.enumerate().cycle() {
        if num_iter == 0 {
            if let Some(text) = text {
                iter(w, text.split("\n"))?;
            };
        }
        if i == 0 {
            w.flush()?;
            if num_iter != 0 {
                selected = input::wait_for_cmdchar()?;
            }
            num_iter += 1;
        }

        if num_iter == 1 {
            queue!(w, Print(format!("{}: ", cmds[i])))?;
            iter(w, item.to_string().split("\n"))?;
        } else {
            if cmds[i] == selected {
                return Ok(item);
            }
        }
    }

    unreachable!()
}

pub fn select_cmd<'a, W: Write, D: Clone + Command + 'a, I: Iterator<Item = &'a D>>(
    w: &mut W,
    text: &str,
    options: I,
) -> Result<D> {
    iter(w, text.split("\n"))?;

    let mut results: Vec<&D> = Vec::new();
    for x in options {
        line(w, x.display_as_cmd())?;
        results.push(x);
    }
    w.flush()?;

    loop {
        let selected = input::wait_for_cmdchar()?;
        for x in results.iter() {
            if x.get_char() == selected {
                let res = (*x).clone();
                return Ok(res);
            }
        }
    }
}

pub fn fzf_search(opts: &str) -> Result<String> {
    use std::io::Read;
    use std::process::{Command, Stdio};

    // Start the fzf process with stdin and stdout pipes
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Get a handle to the stdin and stdout of the fzf process
    let fzf_stdin = fzf.stdin.as_mut().ok_or(Error::ExternalCmdError)?;
    let fzf_stdout = fzf.stdout.as_mut().ok_or(Error::ExternalCmdError)?;

    // Write the input to the stdin of the fzf process
    fzf_stdin.write_all(opts.as_bytes())?;

    let mut res = String::new();
    fzf_stdout.read_to_string(&mut res)?;
    Ok(res.trim().to_string())
}
