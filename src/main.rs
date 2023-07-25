use std::{time::Duration, collections::HashMap};

use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect::Rect, mouse::MouseButton};

fn main() {
    let window_width: u32 = 1600;
    let window_height: u32 = 900;
    let scope = (0..window_width as i32, 0..window_height as i32);

    let tile_size: u32 = 20;
    let mut board: HashMap<(i32, i32), bool> = HashMap::new();
    let mut running = false;

    let mut frame_count: u32 = 0;

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
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    board = HashMap::new();
                    running = false;
                    frame_count = 0;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if running {continue;}

                    let i = x / tile_size as i32;
                    let j = y / tile_size as i32;
                    if board.get(&(i, j)).is_none() {
                        board.insert((i, j), true);
                    }
                    else {
                        board.remove(&(i, j));
                    }
                },
                _ => {}
            }
        }

        //Mechanic
        if running && frames_per_second(frame_count, 2) {
            let board_state = board.clone();
            for ((x, y), _) in board_state.clone() {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let i = x + dr;
                        let j = y + dc;

                        let adjacent = count_adjacent(board_state.clone(), i, j);

                        if board_state.get(&(i, j)).is_some() && adjacent < 2 {
                            board.remove(&(i, j));
                        }
                        else if board_state.get(&(i, j)).is_some() && ((2..=3).contains(&adjacent)){
                            continue;
                        }
                        else if board_state.get(&(i, j)).is_some() && adjacent > 2 {
                            board.remove(&(i, j));
                        }
                        else if board_state.get(&(i, j)).is_none() && adjacent == 3 {
                            board.insert((i, j), true);
                        }
                    }
                }

                //doesn't count for not existing cells
                let adjacent = count_adjacent(board_state.clone(), x, y);

                    if board_state.get(&(x, y)).is_some() && adjacent < 2 {
                        board.remove(&(x, y));
                    }
                    else if board_state.get(&(x, y)).is_some() && ((2..=3).contains(&adjacent)){
                        continue;
                    }
                    else if board_state.get(&(x, y)).is_some() && adjacent > 2 {
                        board.remove(&(x, y));
                    }
                    else if board_state.get(&(x, y)).is_none() && adjacent == 3 {
                        board.insert((x, y), true);
                    }
            }
        }

        //Background
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        //Entities
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        let offset: u32 = 2;
        for ((x, y), _) in board.clone(){
            if scope.0.contains(&x) && scope.1.contains(&y){
                canvas.fill_rect(
                    Rect::new(
                        (x as u32 * tile_size + offset) as i32, 
                        (y as u32 * tile_size + offset) as i32, 
                        tile_size - offset * 2, 
                        tile_size - offset * 2)
                ).unwrap();
            }
        }
        
        canvas.present();

        if frame_count == 60 {frame_count = 0;}
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn count_adjacent(board: HashMap<(i32, i32), bool>, x0: i32, y0: i32) -> i32 {
    let mut count: i32 = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }

            if board.get(&(x0 + dr, y0 + dc)).is_some() {
                count += 1
            }
        }
    }

    return count;
}

fn frames_per_second(frame_count: u32, frames: u32) -> bool {
    return frame_count % (60 / frames) == 0;
}