use futures::executor::block_on;
use std::{
    cell::RefCell,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
    rc::Rc,
};

use richter::client::render::Extent2d;

use chrono::Utc;

const BYTES_PER_PIXEL: u32 = 4;

/// Implements the "screenshot" command.
///
/// This function returns a boxed closure which sets the `screenshot_path`
/// argument to `Some` when called.
pub fn cmd_screenshot(
    screenshot_path: Rc<RefCell<Option<PathBuf>>>,
) -> Box<dyn Fn(&[&str]) -> String> {
    Box::new(move |args| {
        let path = match args.len() {
            // TODO: make default path configurable
            0 => PathBuf::from(format!("richter-{}.png", Utc::now().format("%FT%H-%M-%S"))),
            1 => PathBuf::from(args[0]),
            _ => {
                log::error!("Usage: screenshot [PATH]");
                return "Usage: screenshot [PATH]".to_owned();
            }
        };

        screenshot_path.replace(Some(path));
        String::new()
    })
}

pub struct Capture {
    // size of the capture image
    capture_size: Extent2d,

    // width of a row in the buffer, must be a multiple of 256 for mapped reads
    row_width: u32,

    // mappable buffer
    buffer: wgpu::Buffer,
}

impl Capture {
    pub fn new(device: &wgpu::Device, capture_size: Extent2d) -> Capture {
        // bytes_per_row must be a multiple of 256
        // 4 bytes per pixel, so width must be multiple of 64
        let row_width = (capture_size.width + 63) / 64 * 64;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("capture buffer"),
            size: (row_width * capture_size.height * BYTES_PER_PIXEL) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        Capture {
            capture_size,
            row_width,
            buffer,
        }
    }

    pub fn copy_from_texture(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        texture: wgpu::ImageCopyTexture,
    ) {
        encoder.copy_texture_to_buffer(
            texture,
            wgpu::ImageCopyBuffer {
                buffer: &self.buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.row_width * BYTES_PER_PIXEL),
                    rows_per_image: None,
                },
            },
            self.capture_size.into(),
        );
    }

    pub fn write_to_file<P>(&self, device: &wgpu::Device, path: P)
    where
        P: AsRef<Path>,
    {
        let data = Vec::new();
        {
            let slice = self.buffer.slice(..); // Slice the whole buffer
            let (tx, rx) = futures::channel::oneshot::channel(); // Create a oneshot channel for async communication

            // Start the mapping process
            slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());

            // You need to keep the device active until the mapping is complete
            device.poll(wgpu::Maintain::Wait);

            // Wait for the mapping to be complete
            match block_on(rx) {
                Ok(Ok(())) => {
                    // Mapping was successful, proceed with getting mapped range and the rest of the code
                }
                Ok(Err(e)) => {
                    // Handle the BufferAsyncError if mapping failed
                    eprintln!("Buffer mapping failed: {:?}", e);
                    return;
                }
                Err(_) => {
                    // The oneshot channel was cancelled, handle this error case
                    eprintln!("Oneshot channel was cancelled.");
                    return;
                }
            }
        }

        // Write data to file
        let file = File::create(path).expect("Failed to create file");
        let writer = BufWriter::new(file);
        let mut encoder =
            png::Encoder::new(writer, self.capture_size.width, self.capture_size.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut png_writer = encoder.write_header().expect("Failed to write PNG header");
        png_writer
            .write_image_data(&data)
            .expect("Failed to write PNG data");
    }
}
