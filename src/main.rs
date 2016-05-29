extern crate goagain;
extern crate gtk;
extern crate cairo;

use gtk::prelude::*;

use goagain::goban;

fn draw_board(w : &gtk::DrawingArea, ctx : &cairo::Context) -> Inhibit {
    let height = w.get_allocated_height() as f64;
    let width = w.get_allocated_width() as f64;
    let s = std::cmp::min(
        w.get_allocated_height(),
        w.get_allocated_width()
    ) as f64;
    ctx.set_source_rgb(0.78, 0.86, 0.52);
    ctx.rectangle((width - s)/2.0, (height-s)/2.0, s, s);
    ctx.fill();
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
