use crate::sdl2::render::Texture;

pub struct Ppu<'a> {
    screen: &'a Texture<'a>
}