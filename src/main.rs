use std::time::Duration;

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect::Rect, mouse::MouseButton};

#[derive(Clone)]
struct Cell {
    alive: bool
}

impl Cell {
    fn new(alive: bool) -> Cell {
        return Cell {alive};
    }
}

#[derive(Clone)]
struct Board {
    width: u32,
    height: u32,
    value: Vec<Vec<Cell>>
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        return Board {
            width,
            height,
            value: vec![vec![Cell::new(false); height as usize]; width as usize]
        };
    }
}

fn main() {
    let window_width: u32 = 1600;
    let window_height: u32 = 900;

    let tile_size: u32 = 20;
    let mut board: Board = Board::new(window_width / tile_size, window_height / tile_size);
    let mut running = false;

    let mut frame_count = 0;

    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window("Convey's Game Of Life", window_width, window_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        frame_count += 1;

        //Event handler
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    running = !running;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if running {continue;}

                    let i = (x / tile_size as i32) as usize;
                    let j = (y / tile_size as i32) as usize;
                    board.value[i][j].alive = !board.value[i][j].alive;
                },
                _ => {}
            }
        }

        //Mechanic
        if running && times_per_second(frame_count, 10) {
            let board_state = board.clone();
            for (x, column) in board_state.clone().value.iter().enumerate() {
                for y in 0..column.len(){
                    let adjacent = count_adjacent(board_state.clone(), x as i32, y as i32);

                    if board_state.value[x][y].alive && adjacent < 2 {
                        board.value[x][y].alive = false;
                    }
                    else if board_state.value[x][y].alive && ((2..=3).contains(&adjacent)){
                        continue;
                    }
                    else if board_state.value[x][y].alive && adjacent > 2 {
                        board.value[x][y].alive = false;
                    }
                    else if !board_state.value[x][y].alive && adjacent == 3 {
                        board.value[x][y].alive = true;
                    }
                }
            }
        }

        //Background
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        //Entities
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        let offset: u32 = 2;
        for (x, column) in board.value.iter().enumerate() {
            for (y, tile) in column.iter().enumerate(){
                if (*tile).alive {
                    canvas.fill_rect(
                        Rect::new(
                            (x as u32 * tile_size + offset) as i32, 
                            (y as u32 * tile_size + offset) as i32, 
                            tile_size - offset * 2, 
                            tile_size - offset * 2)
                    ).unwrap();
                }
            }
        }
        
        canvas.present();

        if frame_count == 60 {frame_count = 0;}
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn count_adjacent(board: Board, x0: i32, y0: i32) -> i32 {
    let max_x = (board.width - 1) as i32;
    let max_y = (board.height - 1) as i32;
    let mut count: i32 = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            let mut x: i32 = x0 + dr;
            let mut y: i32 = y0 + dc;

            if dr == 0 && dc == 0 { continue; }

            //wrap out of range indices
            if x < 0 { x = max_x; }
            else if x > max_x { x = 0; }

            if y < 0 { y = max_y; }
            else if y > max_y { y = 0; }

            if board.value[x as usize][y as usize].alive {
                count += 1
            }
        }
    }
    
    return count;
}

fn times_per_second(frame_count: u32, times: u32) -> bool {
    return frame_count % (60 / times) == 0;
}