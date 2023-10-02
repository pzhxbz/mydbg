#[derive(Default)]
pub struct DbgApp {
    name: String,
    age: u32,
}

#[derive(Default)]
pub struct MainWindow {
    app: DbgApp,
}

impl eframe::epi::App for DbgApp {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::u32(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "dbg_gui"
    }
}

impl MainWindow {
    pub fn new() -> MainWindow {
        MainWindow {
            app: DbgApp::default(),
        }
    }

    pub fn run(&self) {
        eframe::run_native(Box::new(DbgApp::default()));
    }
}
