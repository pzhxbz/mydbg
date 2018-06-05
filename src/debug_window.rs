use gtk;
use gtk::{ButtonExt, GtkWindowExt, WidgetExt,ContainerExt};
use gtk::{Button, Window, WindowType};
use gdk;
//use gdk::{ScreenExt};
pub struct MainWindow {
    mainwindow: Window,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow {
            mainwindow: Window::new(WindowType::Toplevel),
        }
    }
    pub fn init(&self) {   
        self.mainwindow.set_title("moe debuger");
        //let screen = gdk::Screen::get_default().unwrap();
        self.mainwindow.set_default_size(gdk::Screen::width(),gdk::Screen::height());
        let button = Button::new_with_label("Click me!");
        self.mainwindow.add(&button);
        self.mainwindow.show_all();

        self.mainwindow.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        button.connect_clicked(|_| {
            //println!("Clicked!");
        });
    }
    pub fn run(&self) {
        gtk::main();
    }
   
}
