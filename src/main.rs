extern crate goagain;
extern crate gtk;
extern crate cairo;

use gtk::prelude::*;

use goagain::goban;

fn draw_board(w : &gtk::DrawingArea, ctx : &cairo::Context) -> Inhibit {
    let lines = 19; //should be parameterzied.

    let height = w.get_allocated_height() as f64;
    let width = w.get_allocated_width() as f64;
    let s = std::cmp::min(
        w.get_allocated_height(),
        w.get_allocated_width()
    ) as f64;
    let x = (width - s)/2.0;
    let y = (height - s)/2.0;

    //Draw board
    ctx.set_source_rgb(0.78, 0.86, 0.52);
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
                    radius, radius, 4.0 * 3.14
                );
                ctx.fill();
            }
        }
    }

    Inhibit(false)
}

fn main() {
    gtk::init().unwrap();
    // Create the main window.
    let builder = gtk::Builder::new_from_file("interface.glade");
    let window : gtk::Window = builder.get_object("window1").unwrap();
    let board : gtk::DrawingArea = builder.get_object("drawingarea1").unwrap();

    board.connect_draw(draw_board);

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
