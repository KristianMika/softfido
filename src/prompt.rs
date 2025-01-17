// Copyright: Helmut Eller
// SPDX-License-Identifier: GPL-3.0-or-later

use pinentry_rs;
use std::sync::mpsc::{Receiver};
use secstr;

// FIXME: report report this as bug pinentry_rs maintainer.
fn escape_string(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\n' => r.push_str("%0A"),
            c => r.push(c)
        }
    };
    r
}

pub fn yes_or_no_p(_prompt: &str)
                  -> Receiver<Result<bool, pinentry_rs::Error>> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let _ = sender.send(Ok(true));
    receiver
}

pub fn read_pin(prompt: &str) -> Result<secstr::SecStr, pinentry_rs::Error> {
    pinentry_rs::pinentry()
        .description(escape_string(prompt))
        .pin("".to_string())
}
