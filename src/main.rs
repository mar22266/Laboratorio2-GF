use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: usize = 30;  

fn initialize_game_of_life(framebuffer: &mut Framebuffer) {
    let patterns = [
        (&GLIDER, (1, 1)),
        (&BEE_HIVE, (10, 1)),
        (&LOAF, (20, 1)),
        (&BOAT, (30, 1)),
        (&TUB, (40, 1)),
        (&BLINKER, (50, 1)),
        (&TOAD, (60, 1)),
        (&BEACON, (70, 1)),
        (&LWSS, (80, 1)),
        (&BLOCK, (90, 1)),
    ];

    for &(pattern, offset) in &patterns {
        initialize_pattern(framebuffer, pattern, offset);
    }
}
const GLIDER: &[(usize, usize)] = &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
const BEE_HIVE: &[(usize, usize)] = &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
const LOAF: &[(usize, usize)] = &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
const BOAT: &[(usize, usize)] = &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
const TUB: &[(usize, usize)] = &[(1, 0), (0, 1), (2, 1), (1, 2)];
const BLINKER: &[(usize, usize)] = &[(1, 0), (1, 1), (1, 2)];
const TOAD: &[(usize, usize)] = &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
const BEACON: &[(usize, usize)] = &[(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)];
const LWSS: &[(usize, usize)] = &[(1, 0), (4, 0), (0, 1), (0, 2), (4, 2), (0, 3), (1, 3), (2, 3), (3, 3)];
const BLOCK: &[(usize, usize)] = &[(0, 0), (1, 0), (0, 1), (1, 1)];

fn initialize_pattern(framebuffer: &mut Framebuffer, pattern: &[(usize, usize)], offset: (usize, usize)) {
    for &(x, y) in pattern {
        for dx in 0..CELL_SIZE {
            for dy in 0..CELL_SIZE {
                framebuffer.set_pixel(x * CELL_SIZE + dx + offset.0, y * CELL_SIZE + dy + offset.1, 0xFFFFFF);
            }
        }
    }
}

fn render(framebuffer: &mut Framebuffer) {
    let mut next_state = vec![0; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            let alive = framebuffer.buffer[idx] == 0xFFFFFF;
            let neighbors = count_neighbors(x, y, &framebuffer);

            if alive && (neighbors < 2 || neighbors > 3) {
                next_state[idx] = 0x000000; // Cell dies
            } else if !alive && neighbors == 3 {
                next_state[idx] = 0xFFFFFF; // Cell is born
            } else {
                next_state[idx] = framebuffer.buffer[idx]; // Cell continues in its current state
            }
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            framebuffer.set_pixel(x, y, next_state[idx]);
        }
    }
}

fn count_neighbors(x: usize, y: usize, framebuffer: &Framebuffer) -> usize {
    let mut count = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for (dx, dy) in directions.iter() {
        let nx = (x as isize + dx + WIDTH as isize) % WIDTH as isize;
        let ny = (y as isize + dy + HEIGHT as isize) % HEIGHT as isize;
        let idx = (ny as usize) * WIDTH + nx as usize;
        if framebuffer.buffer[idx] == 0xFFFFFF {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT, 0x000000);

    initialize_game_of_life(&mut framebuffer);

    let mut window = Window::new(
        "Conway's Game of Life - Render Loop Example",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let frame_delay = Duration::from_millis(16);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut framebuffer);
        window.update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT).unwrap();
        std::thread::sleep(frame_delay);
    }
}
