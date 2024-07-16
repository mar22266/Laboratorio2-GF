use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_SIZE: usize = 20;  // Tamaño de cada célula para mayor visibilidad

fn initialize_game_of_life(framebuffer: &mut Framebuffer) {
    let patterns = [
        (&GLIDER, (10, 10)),
        (&BEE_HIVE, (50, 50)),
        (&LOAF, (90, 90)),
        (&BOAT, (130, 130)),
        (&TUB, (170, 170)),
        (&BLINKER, (210, 210)),
        (&TOAD, (250, 250)),
        (&BEACON, (290, 290)),
        (&LWSS, (330, 330)),
        (&BLOCK, (370, 370)),
    ];

    for &(pattern, position) in &patterns {
        initialize_pattern(framebuffer, pattern, position);
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

fn initialize_pattern(framebuffer: &mut Framebuffer, pattern: &[(usize, usize)], position: (usize, usize)) {
    for &(x, y) in pattern {
        for dx in 0..CELL_SIZE {
            for dy in 0..CELL_SIZE {
                let px = x * CELL_SIZE + dx + position.0;
                let py = y * CELL_SIZE + dy + position.1;
                framebuffer.set_pixel(px, py, 0xFFFFFF); // Set cells as alive (white)
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

            // Apply Conway's Game of Life rules
            if alive && (neighbors == 2 || neighbors == 3) {
                next_state[idx] = 0xFFFFFF; // Cell survives
            } else if !alive && neighbors == 3 {
                next_state[idx] = 0xFFFFFF; // Cell is born
            } else {
                next_state[idx] = 0x000000; // Cell dies or remains dead
            }
        }
    }

    // Apply the next state to the framebuffer
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            if next_state[idx] != framebuffer.buffer[idx] {
                framebuffer.set_pixel(x, y, next_state[idx]);
            }
        }
    }
}

fn count_neighbors(x: usize, y: usize, framebuffer: &Framebuffer) -> usize {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue; }
            let nx = x as i32 + i - 1;
            let ny = y as i32 + j - 1;
            if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                let idx = (ny as usize) * WIDTH + nx as usize;
                if framebuffer.buffer[idx] == 0xFFFFFF {
                    count += 1;
                }
            }
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

    let frame_delay = Duration::from_millis(100); // Timing for animation visibility
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        render(&mut framebuffer);
        window.update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT).unwrap();

        while now.elapsed() < frame_delay {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
