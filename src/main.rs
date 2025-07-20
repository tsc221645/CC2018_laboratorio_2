use raylib::prelude::*;

// Framebuffer struct in RAM memory
pub struct Framebuffer {
    width:  u32,
    height: u32,
    color_buffer: Image,   
    current_color: Color,
    background_color: Color,
}

//patterns definition ---------------------------------------------------------------
pub const BLOCK: &[(usize, usize)] = &[
    (0,0),(1,0),
    (0,1),(1,1),
];

pub const BEEHIVE: &[(usize, usize)] = &[
          (1,0),(2,0),
    (0,1),        (3,1),
          (1,2),(2,2),
];

pub const LOAF: &[(usize, usize)] = &[
          (1,0),(2,0),
    (0,1),        (3,1),
          (1,2),      (3,2),
              (2,3),
];

pub const BOAT: &[(usize, usize)] = &[
    (0,0),(1,0),
    (0,1),      (2,1),
          (1,2),
];

pub const TUB: &[(usize, usize)] = &[
          (1,0),
    (0,1),      (2,1),
          (1,2),
];

pub const BLINKER: &[(usize, usize)] = &[
    (0,0),(1,0),(2,0),          // ← periodo 2
];

pub const TOAD: &[(usize, usize)] = &[
        (1,0),(2,0),(3,0),
    (0,1),(1,1),(2,1),          // ← periodo 2
];

pub const BEACON: &[(usize, usize)] = &[
    (0,0),(1,0),
    (0,1),(1,1),                // ← periodo 2
                (2,2),(3,2),
                (2,3),(3,3),
];

//pulsar
pub const PULSAR: &[(usize, usize)] = &[
    (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
    (0,2),(5,2),(7,2),(12,2),
    (0,3),(5,3),(7,3),(12,3),
    (0,4),(5,4),(7,4),(12,4),
    (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
    (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
    (0,8),(5,8),(7,8),(12,8),
    (0,9),(5,9),(7,9),(12,9),
    (0,10),(5,10),(7,10),(12,10),
    (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
];


pub const PENTA_DECATHLON: &[(usize, usize)] = &[
    (1,0),
    (0,1),(1,1),(2,1),
    (1,2),
    (1,3),
    (1,4),
    (0,5),(1,5),(2,5),
    (1,6),
    (1,7),
];

// ── SPACESHIPS ────────────────────────────────────────────────
pub const GLIDER: &[(usize, usize)] = &[
        (1,0),
              (2,1),
    (0,2),(1,2),(2,2),
];

pub const LWSS: &[(usize, usize)] = &[
        (1,0),            (4,0),
    (0,1),
    (0,2),                     (4,2),
    (0,3),(1,3),(2,3),(3,3),(4,3),
];

pub const MWSS: &[(usize, usize)] = &[
        (1,0),            (4,0),
    (0,1),
    (0,2),                     (5,2),
    (0,3),(1,3),(2,3),(3,3),(4,3),(5,3),
];

pub const HWSS: &[(usize, usize)] = &[
        (1,0),            (4,0),
    (0,1),
    (0,2),                          (6,2),
    (0,3),(1,3),(2,3),(3,3),(4,3),(5,3),(6,3),
];


impl Framebuffer {
    // Specify frame buffer size
    pub fn new(width: u32, height: u32) -> Self {
        let img = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Self {
            width,
            height,
            color_buffer: img,
            current_color: Color::WHITE,
            background_color: Color::BLACK,
        }
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, pos: Vector2) {
        //check point being pressent in the area
        if pos.x >= 0.0 && pos.x < self.width as f32 &&
           pos.y >= 0.0 && pos.y < self.height as f32 {
            self.color_buffer.draw_pixel_v(pos, self.current_color);
        }
    }
    // buffer swap occurs
    pub fn swap_buffers(&mut self,
                        window: &mut RaylibHandle,
                        rl_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(rl_thread, &self.color_buffer) {
            let mut d = window.begin_drawing(rl_thread);
            d.draw_texture(&texture, 5, 5, Color::MAGENTA);
        }
    }

    //captures frame buffer
    pub fn save_png(&self, path: &str) {
        self.color_buffer.export_image(path);
    }
}

// variables to modify grid and window
const GRID_W: usize = 100;          
const GRID_H: usize = 100;          
const WIN_W: i32 = GRID_W as i32;   
const WIN_H: i32 = GRID_H as i32;   
const FPS_DELAY_MS: u64 = 100;   // FPS Delay

// use x and y
#[inline]
fn idx(x: usize, y: usize) -> usize {
    y * GRID_W + x
}

// calculates nearest neighbors that are alive
fn live_neighbors(x: usize, y: usize, buf: &[bool]) -> u8 {
    let mut count = 0;
    for dy in [-1isize, 0, 1] {
        for dx in [-1isize, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = ((x as isize + dx + GRID_W as isize) % GRID_W as isize) as usize;
            let ny = ((y as isize + dy + GRID_H as isize) % GRID_H as isize) as usize;
            if buf[idx(nx, ny)] {
                count += 1;
            }
        }
    }
    count
}

// 4 conway rules
fn step(current: &[bool], next: &mut [bool]) {
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let alive = current[idx(x, y)];
            let n = live_neighbors(x, y, current);
            next[idx(x, y)] = match (alive, n) {
                (true, 2) | (true, 3) => true,  // survives
                (true, _) => false,              // dies on established params
                (false, 3) => true,              // reproduces
                _ => false,
            };
        }
    }
}

/// draws grid in frameburrer using pint
fn draw_grid(fb: &mut Framebuffer, cells: &[bool]) {
    fb.set_current_color(Color::BLACK); // changes background color for dead cells
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let alive = cells[idx(x, y)];
            if alive {
                fb.set_current_color(Color::WHITE);
            } else {
                fb.set_current_color(Color::BLACK);
            }
            fb.point(Vector2::new(x as f32, y as f32));
        }
    }
}

/// Coloca un patrón en (ox, oy) — se envuelve si se sale del borde.
fn place(buf: &mut [bool], pattern: &[(usize, usize)], ox: usize, oy: usize) {
    for &(x, y) in pattern {
        let gx = (x + ox) % GRID_W;
        let gy = (y + oy) % GRID_H;
        buf[idx(gx, gy)] = true;
    }
}


fn load_pattern(buf: &mut [bool]) {
    
    buf.fill(false);                        

    place(buf, BLOCK,  2,  2);
    place(buf, BEEHIVE, 8,  2);
    place(buf, LOAF,   14,  2);
    place(buf, BOAT,   20,  2);
    place(buf, TUB,    26,  2);

    place(buf, BLINKER, 10, 30);
    place(buf, BLINKER, 90, 90);
    place(buf, TOAD,    20, 30);
    place(buf, TOAD,    10, 10);
    place(buf, TOAD,    50, 30);
    place(buf, TOAD,    50, 60);
    place(buf, BEACON,  30, 28);
    place(buf, PULSAR,  50, 5);
    place(buf, PULSAR,  30, 55);
    place(buf, PENTA_DECATHLON, 70, 20);
    place(buf, PENTA_DECATHLON, 90, 0);

    place(buf, GLIDER,  0,  0);
    place(buf, GLIDER,  80,  90);
    place(buf, GLIDER,  2,  60);
    place(buf, LWSS,   80,  80);
    place(buf, LWSS,   50,  5);
    place(buf, LWSS,   80,  5);
    place(buf, LWSS,   20,  70);
    place(buf, MWSS,   40,  80);
    place(buf, HWSS,   60,  60);
    
}

//main function
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("Conway Game of Life – Raylib")
        .build();

    //pixel is equal to 1 cell
    let mut fb = Framebuffer::new(GRID_W as u32, GRID_H as u32);
    fb.set_background_color(Color::BLACK); 
    let mut current = vec![false; GRID_W * GRID_H];
    let mut next = vec![false; GRID_W * GRID_H];
    load_pattern(&mut current);          

    //rendering
    while !rl.window_should_close() {
        step(&current, &mut next);
        draw_grid(&mut fb, &next);
        fb.swap_buffers(&mut rl, &thread);
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            fb.save_png("screenshot.png");
        }
        std::thread::sleep(std::time::Duration::from_millis(FPS_DELAY_MS));
        std::mem::swap(&mut current, &mut next);
    }
}
