use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::windows::io::FromRawHandle;

use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::fileapi::CreateFileA;
use winapi::um::fileapi::OPEN_EXISTING;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::wincon::{ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT};
use winapi::um::winnt::PCSTR;
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE};

use rtoolbox::fix_line_issues::fix_line_issues;
use rtoolbox::safe_string::SafeString;

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

struct HiddenInput {
    mode: u32,
    handle: HANDLE,
}

impl HiddenInput {
    fn new(handle: HANDLE) -> io::Result<HiddenInput> {
        let mut mode = 0;
        let mode_ptr: *mut u32 = &mut mode as *mut u32;

        // Get the old mode so we can reset back to it when we are done
        if unsafe { GetConsoleMode(handle, mode_ptr) } == 0 {
            return Err(std::io::Error::last_os_error());
        }

        // We want to be able to read line by line, and we still want backspace to work
        let new_mode_flags = ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT;
        if unsafe { SetConsoleMode(handle, new_mode_flags) } == 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(HiddenInput { mode, handle })
    }

    fn set_mode(&mut self, line_input: bool) -> io::Result<()> {
        let new_mode_flags = if line_input {
            ENABLE_PROCESSED_INPUT
        } else {
            ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT
        };

        if unsafe { SetConsoleMode(self.handle, new_mode_flags) } == 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for HiddenInput {
    fn drop(&mut self) {
        unsafe {
            SetConsoleMode(self.handle, self.mode as u32);
        }
    }
}

/// Reads a password from the TTY
pub fn read_password() -> std::io::Result<String> {
    let handle = unsafe {
        CreateFileA(
            b"CONIN$\x00".as_ptr() as PCSTR,
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            INVALID_HANDLE_VALUE,
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
    }

    let mut stream = BufReader::new(unsafe { std::fs::File::from_raw_handle(handle as _) });
    read_password_from_handle_with_hidden_input(&mut stream, handle)
}

/// Reads a password from a given file handle
fn read_password_from_handle_with_hidden_input(reader: &mut impl BufRead, handle: HANDLE) -> io::Result<String> {
    let mut password = SafeString::new();

    let mut hidden_input = HiddenInput::new(handle)?;
    hidden_input.set_mode(false).unwrap();

    let reader_return = reader.read_line(&mut password);

    // Newline for windows which otherwise prints on the same line.
    println!();

    if reader_return.is_err() {
        return Err(reader_return.unwrap_err());
    }

    std::mem::drop(hidden_input);

    fix_line_issues(password.into_inner())
}

pub fn read_password_masked<F>(mask: F) -> std::io::Result<String>
where
    F: Fn() -> char,
{
    let handle = unsafe {
        CreateFileA(
            b"CONIN$\x00".as_ptr() as PCSTR,
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            INVALID_HANDLE_VALUE,
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
    }

    let mut password = SafeString::new();

    let mut hidden_input = HiddenInput::new(handle)?;
    hidden_input.set_mode(true).unwrap();

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut mask_widths = Vec::new();

    loop {
        let mut buf = [0u8; 1];

        stdin.read_exact(&mut buf)?;
        let ch = buf[0] as char;

        match ch {
            '\r' | '\n' => {
                stdout.write_all(b"\n")?;
                break;
            }
            '\x08' | '\x7f' => {
                password.pop();
                if let Some(width) = mask_widths.pop() {
                    for _ in 0..width {
                        stdout.write_all(b"\x08 \x08")?;
                    }
                }
            }
            _ => {
                password.push(ch);
                let mask_char = mask();
                stdout.write_all(mask_char.to_string().as_bytes())?;
                let width = mask_char.width().unwrap_or(1);
                mask_widths.push(width);
            }
        }
        stdout.flush()?;
    }

    std::mem::drop(hidden_input);

    Ok(password.into_inner())
}

pub fn read_password_masked_with_animation<F>(mut mask: F) -> std::io::Result<String>
where
    F: FnMut(usize, &mut dyn Write) -> String,
{
    // Abrimos el handle de consola
    let handle = unsafe {
        CreateFileA(
            b"CONIN$\x00".as_ptr() as PCSTR,
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            INVALID_HANDLE_VALUE,
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error());
    }

    let mut password = SafeString::new();
    let mut hidden_input = HiddenInput::new(handle)?;
    hidden_input.set_mode(true).unwrap();

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut mask_widths = Vec::new();

    let mut pos: usize = 0;

    loop {
        let mut buf = [0u8; 1];
        stdin.read_exact(&mut buf)?;
        let ch = buf[0] as char;

        match ch {
            '\r' | '\n' => {
                stdout.write_all(b"\n")?;
                break;
            }
            '\x08' | '\x7f' => {
                password.pop();

                if let Some(width) = mask_widths.pop() {
                    for _ in 0..width {
                        stdout.write_all(b"\x08 \x08")?;
                    }
                    pos = pos.saturating_sub(1);
                }
            }
            _ => {
                password.push(ch);

                let mask_str = mask(pos, &mut stdout);
                stdout.flush()?;

                let width = mask_str.width();
                mask_widths.push(width);

                pos += 1;
            }
        }
        stdout.flush()?;
    }

    std::mem::drop(hidden_input);

    Ok(password.into_inner())
}
