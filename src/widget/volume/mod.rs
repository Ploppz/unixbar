use std::sync::Arc;
use widget::base::Widget;
use format::data::Format;
use std::sync::mpsc::Sender;

#[cfg(target_os = "linux")] pub mod alsa;
#[cfg(target_os = "linux")] pub use self::alsa::ALSA;

pub struct VolumeState {
    pub volume: f32,
    pub muted: bool,
}


pub struct Volume<F: Fn(VolumeState) -> Format, B: VolumeBackend<F>> {
    updater: Arc<Box<F>>,
    backend: B,
}


impl<F, B> Widget for Volume<F, B> where F: Fn(VolumeState) -> Format + Sync + Send + 'static, B: VolumeBackend<F>  {
    fn current_value(&self) -> Format {
        self.backend.current_value()
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        self.backend.spawn_notifier(tx, self.updater.clone());
    }
}

impl<F, B> Volume<F, B> where F: Fn(VolumeState) -> Format, B: VolumeBackend<F> {
    pub fn new(backend: B, updater: F) -> Box<Volume<F, B>> {
        Box::new(Volume {
            updater: Arc::new(Box::new(updater)),
            backend: backend,
        })
    }
}


pub trait VolumeBackend<F: Fn(VolumeState) -> Format> {
    fn current_value(&self) -> Format;
    fn spawn_notifier(&mut self, tx: Sender<()>, updater: Arc<Box<F>>);
}
