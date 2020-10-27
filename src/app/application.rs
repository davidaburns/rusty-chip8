
use super::sys;

pub struct Application {
    pub title: String,
    system: sys::System
}

impl Application {
    pub fn new(title: &str) -> Application {
        Application {
            title: String::from(title),
            system: sys::System::new(title)
        }
    }

    pub fn run(self) {
        self.system.start_loop(move |_, ui| {
            imgui::Window::new(imgui::im_str!("Hello World"))
                .size([300.0, 110.0], imgui::Condition::FirstUseEver)
                .build(ui, || {
                    ui.text("Hello World from imGUI");
                    ui.separator();
    
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!("Mouse Position: {:.1},{:.1}", mouse_pos[0], mouse_pos[1]));
                });
        });
    }
}