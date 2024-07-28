use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
mod color;
mod bitmap;

use crate::framebuffer::FrameBuffer;
use crate::color::Color;

fn main() {
    // Dimensiones de la ventana
    let window_width = 800;
    let window_height = 600;

    // Dimensiones del framebuffer
    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Ventana de prueba - Esc para cerrar",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    window.limit_update_rate(Some(frame_delay));

    // Variables para animación
    let mut x = 0;
    let mut speed = 1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Preparar variables para renderizado
        if x as usize >= framebuffer_width {
            speed = -1;
        }
        if x <= 0 {
            speed = 1;
        }
        x += speed;

        framebuffer.set_background_color(Color::new(51, 51, 85));
        framebuffer.clear();

        // Dibujar un punto animado
        framebuffer.set_current_color(Color::new(255, 221, 221));
        framebuffer.point(x as usize, 40);

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(
                &framebuffer.buffer.iter().map(|c| c.to_hex()).collect::<Vec<u32>>(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

