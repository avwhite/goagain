extern crate goagain;
extern crate gtk;
extern crate cairo;
extern crate gdk;
extern crate core;

use gtk::prelude::*;

use goagain::goban;
use core::ops::DerefMut;

fn draw_board(w : &gtk::DrawingArea, ctx : &cairo::Context, g : &goban::GameState) -> Inhibit {
    let lines = g.size(); //should be parameterzied.

    let height = w.get_allocated_height() as f64;
    let width = w.get_allocated_width() as f64;
    let s = std::cmp::min(
        w.get_allocated_height(),
        w.get_allocated_width()
    ) as f64;
    let x = (width - s)/2.0;
    let y = (height - s)/2.0;

    //Draw board
    //ctx.set_source_rgb(0.78, 0.86, 0.52);
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.rectangle(x, y, s, s);
    ctx.fill();

    //Draw lines
    let line_diff = s/(lines as f64); 
    let inner_x = x + line_diff/2.0;
    let inner_y = y + line_diff/2.0;
    let inner_s = s - line_diff;

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.set_line_width(1.0);
    for i in 0..lines {
        ctx.move_to(inner_x, inner_y + (i as f64)*line_diff);
        ctx.line_to(inner_x + inner_s, inner_y + (i as f64)*line_diff);

        ctx.move_to(inner_x + (i as f64)*line_diff, inner_y);
        ctx.line_to(inner_x + (i as f64)*line_diff, inner_y + inner_s);
    }
    ctx.stroke();

    //Draw hoshi for known board sizes
    if lines == 19 {
        let radius = line_diff/12.0;
        let coords = vec![3, 9, 15];
        for i in &coords {
            for j in &coords {
                ctx.arc(
                    inner_x + (*i as f64)*line_diff,
                    inner_y + (*j as f64)*line_diff,
                    radius, 0.0, 2.0 * 3.14
                );
                ctx.fill();
            }
        }
    }

    //Draw stones:
    let stone_radius = line_diff/2.0 - (line_diff/50.0);
    for i in 0..lines {
        for j in 0..lines {
            match g.intersection((i,j)) {
                goban::Intersection::Black => {
                    ctx.set_source_rgb(0.0, 0.0, 0.0);
                    ctx.arc(
                        inner_x + (i as f64)*line_diff,
                        inner_y + (j as f64)*line_diff,
                        stone_radius, 0.0, 2.0 * 3.14
                    );
                }
                goban::Intersection::White => {
                    ctx.set_source_rgb(1.0, 1.0, 1.0);
                    ctx.arc(
                        inner_x + (i as f64)*line_diff,
                        inner_y + (j as f64)*line_diff,
                        stone_radius, 0.0, 2.0 * 3.14
                    );
                }
                goban::Intersection::Empty => {
                    //Clear the current path
                    ctx.new_path();
                }
            }
            ctx.fill_preserve();
            ctx.set_source_rgb(0.0, 0.0, 0.0);
            ctx.stroke();
        }
    }

    Inhibit(false)
}

fn click_board(w : &gtk::DrawingArea, event : &gdk::EventButton, g : &mut goban::GameModel) -> Inhibit {
    //Calculate coordinates:
    let (click_x, click_y) = event.get_position();

    let lines = g.current_state().size(); //should be parameterzied.

    let height = w.get_allocated_height() as f64;
    let width = w.get_allocated_width() as f64;
    let s = std::cmp::min(
        w.get_allocated_height(),
        w.get_allocated_width()
    ) as f64;
    let x = (width - s)/2.0;
    let y = (height - s)/2.0;

    let line_diff = s/(lines as f64); 
    let inner_x = x + line_diff/2.0;
    let inner_y = y + line_diff/2.0;

    let click_board_x = click_x - inner_x;
    let click_board_y = click_y - inner_y;

    let coord_x = (click_board_x/line_diff).round() as u32;
    let coord_y = (click_board_y/line_diff).round() as u32;

    if coord_x < lines && coord_y < lines {
        g.make_move((coord_x, coord_y));
        w.queue_draw();
    }

    Inhibit(false)
}


fn main() {
    use std::cell::RefCell;
    use std::rc::Rc;

    gtk::init().unwrap();
    // Create the main window.
    let builder = gtk::Builder::new_from_file("interface.glade");
    let window : gtk::Window = builder.get_object("window1").unwrap();
    let board : gtk::DrawingArea = builder.get_object("drawingarea1").unwrap();
    //256 is BUTTON_PRESS_MASK. It was easier to find the constant value in
    //the source code of some system crate on github than finding a name for
    //the mask in the documentation...
    board.add_events(256);

    let gm : Rc<RefCell<goban::GameModel>> =
        Rc::new(RefCell::new(goban::GameModel::new()));
    gm.borrow_mut().make_move((2,2));
    gm.borrow_mut().make_move((3,2));

    //We create a reference which the draw closure can own.
    let draw_gm = gm.clone();
    board.connect_draw(move |ref da, ref ctx| {
        draw_board(da, ctx, draw_gm.borrow().current_state())
    });

    let click_gm = gm.clone();
    board.connect_button_press_event(move |ref da, ref event| {
        click_board(da, event, click_gm.borrow_mut().deref_mut())
    });

    window.show_all();

    // Handle closing of the window.
    window.connect_delete_event(|_, _| {
        // Stop the main loop.
        gtk::main_quit();
        // Let the default handler destroy the window.
        Inhibit(false)
    });
    // Run the main loop.
    gtk::main();
}
