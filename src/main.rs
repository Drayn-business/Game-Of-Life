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

fn main() {
    let window_width: u32 = 1600;
    let window_height: u32 = 900;

    let tile_size: u32 = 20;
    let mut board: Vec<Vec<Cell>> = vec![vec![Cell::new(false); (window_height / tile_size) as usize]; (window_width / tile_size) as usize];
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
                    if running == true {continue;}

                    let i = (x / tile_size as i32) as usize;
                    let j = (y / tile_size as i32) as usize;
                    board[i][j].alive = !board[i][j].alive;
                },
                _ => {}
            }
        }

        //Mechanic
        if running == true && times_per_second(frame_count, 4) {
            let board_state = board.clone();
            for (x, column) in board_state.clone().iter().enumerate() {
                for y in 0..column.len(){
                    let adjacent = count_adjacent(board_state.clone(), x as i32, y as i32);

                    if board_state[x][y].alive == true && adjacent < 2 {
                        board[x][y].alive = false;
                    }
                    else if board_state[x][y].alive == true && (adjacent == 2 || adjacent == 3){
                        continue;
                    }
                    else if board_state[x][y].alive == true && adjacent > 2 {
                        board[x][y].alive = false;
                    }
                    else if board_state[x][y].alive == false && adjacent == 3 {
                        board[x][y].alive = true;
                    }
                }
            }
        }

        //Background
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        //Entities
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        for (x, column) in board.iter().enumerate() {
            for (y, tile) in column.iter().enumerate(){
                if (*tile).alive == true {
                    canvas.fill_rect(
                        Rect::new((x as u32 * tile_size) as i32 + 2, (y as u32 * tile_size) as i32 + 2, tile_size - 4, tile_size - 4)
                    ).unwrap();
                }
            }
        }
        
        canvas.present();

        if frame_count == 60 {frame_count = 0;}
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn count_adjacent(board: Vec<Vec<Cell>>, x: i32, y: i32) -> i32 {
    let max_x = (board.len() - 1) as i32;
    let max_y = (board[0].len() - 1) as i32;
    let mut count: i32 = 0;

    if y > 0 {
        if x > 0 {
            if board[x as usize - 1][y as usize - 1].alive == true {count += 1;}
        }

        if board[x as usize][y as usize - 1].alive == true {count += 1;}
        
        if x < max_x {
            if board[x as usize + 1][y as usize - 1].alive == true {count += 1;}
        }
    }

    if x > 0 {
        if board[x as usize - 1][y as usize].alive == true {count += 1;}
    }
    if x < max_x {
        if board[x as usize + 1][y as usize].alive == true {count += 1;}
    }

    if y < max_y {
        if x > 0 {
            if board[x as usize - 1][y as usize + 1].alive == true {count += 1;}
        }
        if board[x as usize][y as usize + 1].alive == true {count += 1;}

        if x < max_x {
            if board[x as usize + 1][y as usize + 1].alive == true {count += 1;}
        }
    }

    return count;
}

fn times_per_second(frame_count: u32, times: u32) -> bool {
    return frame_count % (60 / times) == 0;
}