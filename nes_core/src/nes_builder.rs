use crate::nes::{Nes, NESConfig};
use crate::controller::{NESController, DummyController};
use crate::ppu::{VideoInterface, DummyVideo};
use crate::apu::{AudioOutput, DummyAudio};
use crate::cart::Cart;

pub struct NesBuilder<V: VideoInterface, C: NESController, A: AudioOutput> {
    vid: V,
    cont: C,
    audio: A
}

impl<V: VideoInterface, C: NESController, A: AudioOutput> NesBuilder<V,C,A> {
    pub fn video<W: VideoInterface>(self, vid: W) -> NesBuilder<W,C,A> {
        NesBuilder {
            vid,
            cont: self.cont,
            audio: self.audio
        }
    }
    pub fn controller<D: NESController>(self, cont: D) -> NesBuilder<V,D,A> {
        NesBuilder {
            cont,
            vid: self.vid,
            audio: self.audio
        }
    }
    pub fn audio<B: AudioOutput>(self, audio: B) -> NesBuilder<V,C,B> {
        NesBuilder {
            audio,
            vid: self.vid,
            cont: self.cont
        }
    }
    pub fn build<T: Into<Option<Cart>>, U: Into<Option<NESConfig>>>(self, cart: T, config: U) -> Nes<V,C,A> {
        Nes::new(cart.into(), self.vid, self.cont, self.audio, config.into())
    }
}

pub fn nes_builder() -> NesBuilder<DummyVideo, DummyController, DummyAudio> {
    NesBuilder {
        vid: DummyVideo {},
        cont: DummyController {},
        audio: DummyAudio {}
    }
}