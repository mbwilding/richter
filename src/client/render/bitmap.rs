// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use client::render;
use client::render::Palette;
use common::wad::QPic;

use cgmath::Matrix4;
use failure::Error;
use gfx::Factory;
use gfx::format::R8_G8_B8_A8;
use gfx::handle::ShaderResourceView;
use gfx::handle::Texture;
use gfx_device_gl::Resources;

#[derive(Clone, Debug)]
pub struct BitmapTexture {
    width: u32,
    height: u32,
    handle: Texture<Resources, R8_G8_B8_A8>,
    view: ShaderResourceView<Resources, [f32; 4]>,
}

impl BitmapTexture {
    pub fn new<F>(factory: &mut F, width: u32, height: u32, rgba: Box<[u8]>) -> Result<BitmapTexture, Error>
    where
        F: Factory<Resources>,
    {
        let (handle, view) = render::create_texture(factory, width, height, &rgba)?;

        Ok(BitmapTexture {
            width,
            height,
            handle,
            view,
        })
    }

    pub fn from_qpic<F>(factory: &mut F, qpic: &QPic, palette: &Palette) -> Result<BitmapTexture, Error>
    where
        F: Factory<Resources>,
    {
        let (rgba, _fullbright) = palette.translate(qpic.indices());

        BitmapTexture::new(factory, qpic.width(), qpic.height(), rgba.into_boxed_slice())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn view(&self) -> ShaderResourceView<Resources, [f32; 4]> {
        self.view.clone()
    }

    pub fn transform(
        &self,
        display_width: u32,
        display_height: u32,
        position_x: i32,
        position_y: i32,
    ) -> Matrix4<f32> {
        // find center
        let center_x = position_x + self.width as i32 / 2;
        let center_y = position_y + self.height as i32 / 2;

        // rescale from [0, DISPLAY_*] to [-1, 1] (NDC)
        // TODO: this may break on APIs other than OpenGL
        let ndc_x = (center_x * 2 - display_width as i32) as f32 / display_width as f32;
        let ndc_y = (center_y * 2 - display_height as i32) as f32 / display_height as f32;

        let scale_x = self.width as f32 / display_width as f32;
        let scale_y = self.height as f32 / display_height as f32;

        Matrix4::from_translation([ndc_x, ndc_y, 0.0].into())
            * Matrix4::from_nonuniform_scale(scale_x, scale_y, 1.0)
    }
}
