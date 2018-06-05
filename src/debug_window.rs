use gdk;
use gtk;
//use gtk::{Button, Menu,MenuBar, MenuExt, MenuItem, MenuShellExt, Window, WindowType};
use gtk::*;
//use gdk::{ScreenExt};


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

        // let view_menu = create_menus("View", ["nop"].to_vec());
        // let debug_menu = create_menus("Debug", ["nop"].to_vec());
        // let command_menu = create_menus("Command", ["edit"].to_vec());
        // let option_menu = create_menus("Option", ["nop"].to_vec());
        // let help_menu = create_menus("Help", ["about"].to_vec());
        menu_bar.add(&self.init_file_menu());
        menu_bar.add(&self.init_view_menu());
        menu_bar.add(&self.init_debug_menu());
        menu_bar.add(&self.init_command_menu());
        menu_bar.add(&self.init_option_menu());
        menu_bar.add(&self.init_help_menu());
        menu_bar.show_all();
        self.v_box.pack_start(&menu_bar, false, false, 0);
    }
    fn init_file_menu(&self) -> MenuItem {
        let file_menu = SubMenus::new("File");
        let open_submenu = MenuItem::new_with_label("open");
        open_submenu.connect_activate(move |_| {
            // add open file here
        });
        file_menu.append(open_submenu);

        let attach_submenu = MenuItem::new_with_label("attach");
        attach_submenu.connect_activate(move |_| {
            // add attach here
        });
        file_menu.append(attach_submenu);

        let exit_menu = MenuItem::new_with_label("exit");
        exit_menu.connect_activate(move |_| {
            gtk::main_quit();
            gtk::Inhibit(false);
        });
        file_menu.append(exit_menu);

        file_menu.get_menu()
    }

    fn init_view_menu(&self) -> MenuItem {
        let view_menu = SubMenus::new("View");

        let test_submenu = MenuItem::new_with_label("nop");
        view_menu.append(test_submenu);

        view_menu.get_menu()
    }

    fn init_debug_menu(&self) -> MenuItem {
        let debug_menu = SubMenus::new("Debug");

        let test_submenu = MenuItem::new_with_label("nop");
        debug_menu.append(test_submenu);

        debug_menu.get_menu()
    }

    fn init_command_menu(&self) -> MenuItem {
        let command_menu = SubMenus::new("Command");

        let edit_submenu = MenuItem::new_with_label("edit");
        command_menu.append(edit_submenu);

        command_menu.get_menu()
    }

    fn init_option_menu(&self) -> MenuItem {
        let option_menu = SubMenus::new("Option");

        let test_submenu = MenuItem::new_with_label("nop");
        option_menu.append(test_submenu);

        option_menu.get_menu()
    }

    fn init_help_menu(&self) -> MenuItem {
        let help_menu = SubMenus::new("Help");

        let about_submenu = MenuItem::new_with_label("about");
        let main_window_clone=self.mainwindow.clone();
        about_submenu.connect_activate(move |_|{
            let messagebox = MessageDialog::new(Some(&main_window_clone),DialogFlags::USE_HEADER_BAR,MessageType::Info,
            ButtonsType::Ok,"test"
            );
            messagebox.run();
            messagebox.destroy();
        });
        help_menu.append(about_submenu);

        help_menu.get_menu()
    }
}

#[derive(Debug)]
struct SubMenus {
    name: String,
    menu: Menu,
}
impl SubMenus {
    pub fn new(menu_name: &str) -> SubMenus {
        SubMenus {
            name: String::from(menu_name),
            menu: Menu::new(),
        }
    }

    pub fn append(&self, menu: MenuItem) {
        self.menu.append(&menu);
    }

    pub fn get_menu(&self) -> MenuItem {
        let ret_menu = MenuItem::new_with_label(&self.name);
        ret_menu.set_submenu(Some(&self.menu));
        ret_menu
    }
}
