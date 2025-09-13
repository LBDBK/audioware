use kira::sound::{
    PlaybackState, static_sound::StaticSoundHandle, streaming::StreamingSoundHandle,
};

use crate::engine::{traits::stop::Stop, tweens::IMMEDIATELY};

pub mod clear;
pub mod dilation;
pub mod pause;
pub mod playback;
pub mod reclaim;
pub mod resume;
pub mod stop;
pub mod store;

#[derive(Default)]
pub struct Handles<K, V, O>(Vec<Handle<K, V, O>>);

pub struct DualHandles<K, O, E> {
    pub statics: Handles<K, StaticSoundHandle, O>,
    pub streams: Handles<K, StreamingSoundHandle<E>, O>,
}

impl<K, O, E> Default for DualHandles<K, O, E> {
    fn default() -> Self {
        Self {
            statics: Handles(Vec::new()),
            streams: Handles(Vec::new()),
        }
    }
}

pub struct RawHandle<K, V> {
    key: K,
    value: V,
}

impl<K, V> RawHandle<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct Handle<K, V, O> {
    handle: RawHandle<K, V>,
    options: O,
}

impl<K, V, O> Handle<K, V, O> {
    pub fn new(key: K, value: V, options: O) -> Self {
        Self {
            handle: RawHandle::new(key, value),
            options,
        }
    }
}

impl<K> RawHandle<K, StaticSoundHandle> {
    pub fn any_playing_handle(&self) -> bool {
        self.value.state() == PlaybackState::Playing
    }
}

impl<K, E> RawHandle<K, StreamingSoundHandle<E>> {
    pub fn any_playing_handle(&self) -> bool {
        self.value.state() == PlaybackState::Playing
    }
}

impl<K, O, E> DualHandles<K, O, E> {
    pub fn any_playing_handle(&self) -> bool {
        self.statics.0.iter().any(|x| x.handle.any_playing_handle())
            || self.streams.0.iter().any(|x| x.handle.any_playing_handle())
    }
    pub fn any_handle(&self) -> bool {
        !self.statics.0.is_empty() || !self.streams.0.is_empty()
    }
}

impl<K, O, E> Drop for DualHandles<K, O, E> {
    fn drop(&mut self) {
        // bug in kira DecodeScheduler NextStep::Wait
        self.streams.stop(IMMEDIATELY);
    }
}
