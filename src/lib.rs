extern crate chrono;
#[macro_use] extern crate nom;
#[cfg(feature = "systemstat")] extern crate systemstat;
#[cfg(feature = "xkb")] extern crate xcb;
#[cfg(target_os = "linux")] extern crate alsa;
#[cfg(target_os = "linux")] extern crate libc;
extern crate serde;
extern crate serde_json;

pub mod format;
pub mod widget;

use std::sync::mpsc::channel;
pub use format::*;
pub use widget::*;

pub struct UnixBar<F: Formatter> {
    formatter: F,
    widgets: Vec<Box<Widget>>,
}

impl<F: Formatter> UnixBar<F> {
    pub fn new(formatter: F) -> UnixBar<F> {
        UnixBar {
            formatter: formatter,
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: Box<Widget>) -> &mut UnixBar<F> {
        self.widgets.push(widget); self
    }

    pub fn run(&mut self) {
        let (tx, rx) = channel();
        for widget in &mut self.widgets {
            widget.spawn_notifier(tx.clone());
        }
        self.show();
        for _ in rx.iter() {
            self.show();
        }
    }

    fn show(&mut self) {
        let vals : Vec<Format> = self.widgets.iter().map(|ref w| w.current_value()).collect();
        let line = self.formatter.format_all(&vals);
        println!("{}", line.replace("\n", ""));
    }
}
