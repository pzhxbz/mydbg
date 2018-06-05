use gdk;
use gtk;
//use gtk::{Button, Menu,MenuBar, MenuExt, MenuItem, MenuShellExt, Window, WindowType};
use gtk::*;
//use gdk::{ScreenExt}; 

fn create_menus(name: &str, menus: Vec<&str>) -> MenuItem {
    let res = MenuItem::new_with_label(&name);
    let submenu = Menu::new();
    for menu in menus {
        submenu.append(&MenuItem::new_with_label(&menu));
    }
    submenu.show_all();
    res.set_submenu(Some(&submenu));
    res
}

pub struct MainWindow {
    mainwindow: Window,
    v_box: Box,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow {
            mainwindow: Window::new(WindowType::Toplevel),
            v_box: Box::new(Orientation::Vertical, 10),
        }
    }
    pub fn init(&self) {
        self.mainwindow.set_title("moe debuger");
        //let screen = gdk::Screen::get_default().unwrap();
        self.mainwindow
            .set_default_size(gdk::Screen::width(), gdk::Screen::height());
        //let button = Button::new_with_label("Click me!");
        self.init_menu();
        //self.mainwindow.add(&self.init_menu());

        self.v_box.show_all();
        self.mainwindow.add(&self.v_box);
        self.mainwindow.show_all();

        self.mainwindow.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        // button.connect_clicked(|_| {
        //     //println!("Clicked!");
        // });
    }
    pub fn run(&self) {
        gtk::main();
    }

    fn init_menu(&self) {
        let menu_bar = MenuBar::new();

        let file_menu = create_menus("File", ["open","attach","exit"].to_vec());
        let view_menu = create_menus("View", ["nop"].to_vec());
        let debug_menu = create_menus("Debug", ["nop"].to_vec());
        let command_menu = create_menus("Command", ["edit"].to_vec());
        let option_menu = create_menus("Option", ["nop"].to_vec());
        let help_menu = create_menus("Help", ["about"].to_vec());
        menu_bar.add(&file_menu);
        menu_bar.add(&view_menu);
        menu_bar.add(&debug_menu);
        menu_bar.add(&command_menu);
        menu_bar.add(&option_menu);
        menu_bar.add(&help_menu);
        menu_bar.show_all();
        self.v_box.pack_start(&menu_bar, false, false, 0);
    }
}
