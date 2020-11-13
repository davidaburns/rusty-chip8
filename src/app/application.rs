use glium::glutin::dpi::PhysicalSize;
use super::sys;


pub struct ApplicationState {

}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        ApplicationState {

        }
    }
}

pub struct Application {
    title: String,
    system: sys::System,
    state: ApplicationState
}

impl Application {
    pub fn new(title: &str) -> Application {
        Application {
            title: String::from(title),
            system: sys::System::new(title),
            state: ApplicationState::new()
        }
    }

    pub fn run(self) {
        self.system.start_loop(move |_, ui| {
            let window_size = ui.io().display_size;
            let mouse_pos = ui.io().mouse_pos;

            imgui::Window::new(imgui::im_str!("Emulator State"))
                .size([300.0, window_size[1]], imgui::Condition::FirstUseEver)
                .build(ui, || {
                    ui.text("Hello World from imGUI");
                    ui.separator();

                    ui.text(format!("Window Size: {:.1}x{:.1}", window_size[0] as u32, window_size[1] as u32));
                    ui.separator();

                    ui.text(format!("Mouse Position: {:.1},{:.1}", mouse_pos[0], mouse_pos[1]));
                });
        });
    }
}