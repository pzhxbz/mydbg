extern crate gtk;
extern crate radeco_lib;
extern crate gdk;

mod debug_window;
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let window = debug_window::MainWindow::new();
    window.init();
    window.run();
}
