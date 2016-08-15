extern crate piston_window;
extern crate rand;
extern crate find_folder;

use piston_window::*;

mod tetromino;
mod block;
mod board;

use tetromino::*;
use block::*;
use board::*;

const ZOOM: f64 = 30.0;

fn draw_board<G>(board: &Board, transform: math::Matrix2d, g: &mut G)
    where G: Graphics
{
    for (py, line) in board.cells().iter().enumerate() {
        for (px, cell) in line.iter().enumerate() {
            if let Some(color) = *cell {
                rectangle(color.as_rgba(),
                          [(px as f64) * ZOOM, (py as f64) * ZOOM, ZOOM, ZOOM],
                          transform,
                          g);
            }
        }
    }
}

fn draw_rotation<G>(r: &Rotation, color: Color, transform: math::Matrix2d, g: &mut G)
    where G: Graphics
{
    for p in r.iter() {
        let (px, py) = *p;
        rectangle(color.as_rgba(),
                  [px as f64 * ZOOM, py as f64 * ZOOM, ZOOM, ZOOM],
                  transform,
                  g);
    }
}

fn draw_block<G>(b: &Block, transform: math::Matrix2d, g: &mut G)
    where G: Graphics
{
    let (x, y) = b.position();
    draw_rotation(b.rotation(),
                  b.color(),
                  transform.trans(x as f64 * ZOOM, y as f64 * ZOOM),
                  g);
}

fn level(lc: usize) -> usize {
	std::cmp::min(lc / 10, 10) + 1
}

fn main() {
    let mut board = Board::new();

    let mut next_block = Block::new();
    let mut current_block = Some(Block::new());

    let mut line_count = 0;
    let mut freefall_count = 0;
    let mut score = 0;

    let mut pause = false;
    let mut game_over = false;

    let mut time = 0.0;

    let w = (BOARD_WIDTH + 6) * ZOOM as usize;
    let h = BOARD_HEIGHT * ZOOM as usize;

    let mut window: PistonWindow = WindowSettings::new("Tetris", [w as u32, h as u32])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build window: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);

                let t = c.transform.trans(0.5, 0.0);

                draw_board(&board, t, g);

                if let Some(block) = current_block {
                    draw_block(&block, t, g);
                }

                let (w, h) = ((BOARD_WIDTH as f64) * ZOOM, (BOARD_HEIGHT as f64) * ZOOM);

                // draw grid
                grid::Grid {
                        cols: 10,
                        rows: 20,
                        units: ZOOM,
                    }
                    .draw(&line::Line::new([0.1, 0.1, 0.1, 1.0], 0.5),
                          &c.draw_state,
                          t,
                          g);

                Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0)
                    .draw([0.0, 0.0, w, h], &c.draw_state, t, g);

                // draw next block + grid
                let t2 = t.trans(w + ZOOM / 2.0, 0.0);
                draw_rotation(next_block.rotation(),
                              next_block.color(),
                              t2.trans(ZOOM * 2.0, ZOOM),
                              g);
                grid::Grid {
                        cols: 4,
                        rows: 4,
                        units: ZOOM,
                    }
                    .draw(&line::Line::new([0.0, 0.0, 0.0, 1.0], 0.5),
                          &c.draw_state,
                          t2,
                          g);

                // draw text
                use piston_window::text::Text;

                let t2 = t.trans(w + ZOOM / 2.0, h / 2.0 - ZOOM);
                let s = format!("Lines: {}", line_count);
                Text::new_color([1.0, 1.0, 1.0, 1.0], ZOOM as u32)
                    .draw(&s, &mut glyphs, &c.draw_state, t2, g);

                let level = level(line_count);
                let s = format!("Level: {}", level);
                Text::new_color([1.0, 1.0, 1.0, 1.0], ZOOM as u32)
                    .draw(&s, &mut glyphs, &c.draw_state, t2.trans(0.0, ZOOM), g);

                let s = format!("Score: {}", score);
                Text::new_color([1.0, 1.0, 1.0, 1.0], ZOOM as u32)
                    .draw(&s, &mut glyphs, &c.draw_state, t2.trans(0.0, ZOOM * 2.0), g);


                if game_over {
                    Text::new_color([1.0, 0.2, 1.0, 1.0], ZOOM as u32).draw("GAME OVER!",
                                                                            &mut glyphs,
                                                                            &c.draw_state,
                                                                            t2.trans(0.0,
                                                                                     ZOOM * 5.0),
                                                                            g);
                } else if pause {
                    Text::new_color([1.0, 0.3, 1.0, 1.0], ZOOM as u32).draw("PAUSED",
                                                                            &mut glyphs,
                                                                            &c.draw_state,
                                                                            t2.trans(0.0,
                                                                                     ZOOM * 5.0),
                                                                            g);
                }

            });
        }

        if let Some(args) = e.update_args() {
            if !game_over && !pause {
                let level = level(line_count);
                let speed = (11 - level) as f64 * 0.05;

                time += 1.0 / speed * args.dt;

                if time >= 1.0 {

                    if let Some(block) = current_block {
                        if !board.overlap(&block.displace(Direction::Down)) {
                            freefall_count += 1;
                            current_block = Some(block.displace(Direction::Down));
                        } else {
                            score += 21 + (3 * level) - freefall_count;

                            freefall_count = 0;

                            board.merge(&block);

                            line_count += board.remove_lines();

                            current_block = None;
                        }
                    } else {
                        if board.overlap(&next_block) {
                            game_over = true;
                        } else {
                            current_block = Some(next_block);
                            next_block = Block::new();
                        }
                    }

                    time = 0.0;
                }
            }
        }

        if let Some(args) = e.press_args() {
            if !game_over {
                if !pause {
                    if let Some(mut block) = current_block {
                        if args == Button::Keyboard(Key::Left) {
                            if !board.overlap(&block.displace(Direction::Left)) {
                                current_block = Some(block.displace(Direction::Left))
                            }
                        } else if args == Button::Keyboard(Key::Right) {
                            if !board.overlap(&block.displace(Direction::Right)) {
                                current_block = Some(block.displace(Direction::Right))
                            }
                        } else if args == Button::Keyboard(Key::Space) ||
                                  args == Button::Keyboard(Key::Down) {
                            while !board.overlap(&block.displace(Direction::Down)) {
                                block = block.displace(Direction::Down)
                            }
                            current_block = Some(block);
                            time += 1.0
                        } else if args == Button::Keyboard(Key::Up) {
                            if !board.overlap(&block.rotate()) {
                                current_block = Some(block.rotate())
                            }
                        }
                    }
                }
                if args == Button::Keyboard(Key::P) {
                    pause = !pause;
                }
            }
        }
    }
}
