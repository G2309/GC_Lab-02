use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::env;
use image::{GenericImageView, ImageError, RgbImage};

mod framebuffer;
mod color;
mod bitmap;

use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::bitmap::write_bmp_file;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

// Cargar JPG como BMP y convertirlo a estado inicial del tablero
fn load_image_as_board(file_path: &str) -> Result<Vec<Vec<bool>>, ImageError> {
    let img = image::open(file_path)?.to_rgb8();

    // Redimensionar la imagen si es necesario
    let img = if img.width() as usize != WIDTH || img.height() as usize != HEIGHT {
        image::imageops::resize(&img, WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Nearest)
    } else {
        img
    };

    // Crear un tablero basado en la imagen
    let mut board = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = img.get_pixel(x as u32, y as u32);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // Considera una célula viva si el color no es negro (0, 0, 0)
            board[y][x] = !(r == 0 && g == 0 && b == 0);
        }
    }

    Ok(board)
}

// Función para contar células vecinas
fn count_neighbors(board: &Vec<Vec<bool>>, x: isize, y: isize) -> usize {
    let mut count = 0;

    // Revisar células a los alrededores
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = x + i;
            let ny = y + j;
            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                if board[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

// Función para actualizar el estado del tablero
fn update_board(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = vec![vec![false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = board[y][x];
            let neighbors = count_neighbors(board, x as isize, y as isize);

            new_board[y][x] = match (alive, neighbors) {
                (true, n) if n < 2 => false,
                (true, 2) | (true, 3) => true,
                (true, n) if n > 3 => false,
                (false, 3) => true,
                _ => alive,
            };
        }
    }
    new_board
}

fn render(framebuffer: &mut FrameBuffer, board: &Vec<Vec<bool>>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if board[y][x] {
                Color::new(255, 255, 255) // Célula viva = blanco
            } else {
                Color::new(0, 0, 0) // Célula muerta = negro
            };
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    // Imprimir el directorio actual
    println!("Current directory: {:?}", env::current_dir().unwrap());

    // Dimensiones de la ventana
    let window_width = WIDTH;
    let window_height = HEIGHT;

    // Dimensiones del framebuffer
    let framebuffer_width = WIDTH;
    let framebuffer_height = HEIGHT;

    let frame_delay = Duration::from_millis(75);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Game of Life - Esc para cerrar",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    window.limit_update_rate(Some(frame_delay));

    // Puedes cargar la imagen que desees para un tablero inicial, preferiblemente que sea png y
    // recuerda colocar una imagen y su path absoluto para que no de error
    let mut board = load_image_as_board("/home/gustavo/Progra/Lab-02/src/board1.png").expect("Error al cargar la imagen");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Renderizar el framebuffer con el estado actual del tablero
        render(&mut framebuffer, &board);

        // Actualizar el tablero al siguiente estado
        board = update_board(&board);

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

