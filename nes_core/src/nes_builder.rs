use crate::nes::{Nes, NESConfig};
use crate::controller::{NESController, DummyController};
use crate::ppu::{VideoInterface, DummyVideo};
use crate::cart::Cart;

pub struct NesBuilder<V: VideoInterface, C: NESController> {
    vid: V,
    cont: C
}

impl<V: VideoInterface, C: NESController> NesBuilder<V,C> {
    pub fn video<W: VideoInterface>(self, video: W) -> NesBuilder<W,C> {
        NesBuilder {
            cont: self.cont,
            vid: video
        }
    }
    pub fn controller<D: NESController>(self, c: D) -> NesBuilder<V,D> {
        NesBuilder {
            cont: c,
            vid: self.vid
        }
    }
    pub fn build<T: Into<Option<Cart>>, U: Into<Option<NESConfig>>>(self, cart: T, config: U) -> Nes<V,C> {
        Nes::new(cart.into(), self.vid, self.cont, config.into())
    }
}

pub fn nes_builder() -> NesBuilder<DummyVideo, DummyController> {
    NesBuilder {
        vid: DummyVideo {},
        cont: DummyController {}
    }
}