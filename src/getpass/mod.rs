use std::io::Write;

use rtoolbox::print_tty::print_tty;

mod windows;

#[cfg(target_family = "windows")]
pub use windows::{read_password, read_password_masked, read_password_masked_with_animation};

pub fn getpass(prompt: impl ToString) -> Result<String, std::io::Error> {
    print_tty(prompt.to_string().as_str()).and_then(|_| read_password())
}

pub fn getpass_masked<F>(prompt: impl ToString, mask: F) -> Result<String, std::io::Error>
where
    F: Fn() -> char,
{
    print_tty(prompt.to_string().as_str()).and_then(|_| read_password_masked(mask))
}

pub fn getpass_masked_with_animation<F>(prompt: &str, mask: F) -> std::io::Result<String>
where
    F: FnMut(usize, &mut dyn Write) -> String,
{
    print_tty(prompt.to_string().as_str()).and_then(|_| read_password_masked_with_animation(mask))
}

// let emojis = ["🎰", "🍒", "🍋", "🔔", "⭐", "💎", "💰"];
// let delay = Duration::from_millis(8);
// let password = get_password_masked_with_animation("password: ", |pos, out| {
//     for i in 0..15 {
//         let emoji = emojis[rand::thread_rng().gen_range(0..emojis.len())];
//         let width = emoji.width();

//         out.write_all(emoji.as_bytes()).unwrap();
//         out.flush().unwrap();
//         sleep(delay);

//         if i < 14 {
//             for _ in 0..width {
//                 out.write_all(b"\x08 \x08").unwrap();
//             }
//         } else {
//             for _ in 0..width {
//                 out.write_all(b"\x08 \x08").unwrap();
//             }
//             out.write_all("*".as_bytes()).unwrap();
//             out.flush().unwrap();
//         }
//     }
//     "🔒".to_string()
// })
// .unwrap();

// println!("\n🔐 Password entered: {password}");
