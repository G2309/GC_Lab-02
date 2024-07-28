use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
mod color;
mod bitmap;

use crate::framebuffer::FrameBuffer;
use crate::color::Color;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

// Funcion para iniciar un tablero

fn board_init() -> Vec<Vec<bool>> {
    let mut board = vec![vec![false; WIDTH]; HEIGHT];
    
    // Tablero inicial
    board[1][2] = true;
    board[2][3] = true;
    board[3][1] = true;
    board[1][3] = true;
    board[2][2] = true;
    board[3][3] = true;

    board
}

// Funcion para contar celulas vecinas

fn count_neighbors(board: &Vec<Vec<bool>>, x: isize, y:isize) -> usize {
    let mut count = 0;

    // Revisar celulas a los alrededores
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let nx = x+i;
            let ny = y+j;
            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                if board[ny as usize][nx as usize]{
                    count += 1;
                }
            }
        }
    }
    count
}

// Funcion para actualizar el estado del tablero

fn update_board(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = vec![vec![false;WIDTH];HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alive = board[y][x];
            let neighbors = count_neighbors(board, x as isize, y as isize);

            new_board[y][x] = match (alive, neighbors) {
                (true,n) if n < 2 => false,
                (true,2) | (true,3) => true,
                (true,n) if n > 3 => false,
                (false,3) => true,
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
                Color::new(255,255,255) //Celula viva = blanco
            } else {
                Color::new(0,0,0) // Celula muerta = negro
            };
            framebuffer.set_current_color(color);
            framebuffer.point(x,y);
        }
    }
}

fn main() {
    // Dimensiones de la ventana
    let window_width = WIDTH * 8;
    let window_height = HEIGHT * 8;

    // Dimensiones del framebuffer
    let framebuffer_width = WIDTH;
    let framebuffer_height = HEIGHT;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = FrameBuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Game of Life - Esc para cerrar",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Error al crear la ventana: {}", e);
    });

    window.limit_update_rate(Some(frame_delay));

    // Inicializar el tablero
    let mut board = board_init();

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

