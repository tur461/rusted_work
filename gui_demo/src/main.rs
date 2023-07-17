use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use std::cell::RefCell;

const APP_ID: &str = "gui.demo.first_id";

struct Tog(bool);

impl Tog {
    fn new() -> Self {
        Self(false)
    }

    fn toggle(&mut self) {
        if self.0 {
            self.0 = false;
        } else {
            self.0 = true;
        }
    }

    fn is_true(&self) -> bool {
        self.0
    }
}

fn main() -> glib::ExitCode {
    let tog = Tog::new();
    let m_app = MyApp::new(tog);

    m_app.start()
}

struct MyApp {
    tog: Tog,
    app: Application,
}

impl MyApp {
    fn new(tog: Tog) -> Self {
        Self {
            tog,
            app: Application::builder().application_id(APP_ID).build(),
        }
    }

    fn start(&self) -> glib::ExitCode {
        // connect to "activate" signal of the app
        self.app.connect_activate(self.build_ui(&self.app));
        // finally
        self.app.run()
    }

    fn build_ui(&self, app: &Application) -> impl Fn(&Application) {
        let activate_signal_handler = |app: &Application| {
            let btn = Button::builder()
                .label("start!")
                .margin_top(10)
                .margin_bottom(10)
                .margin_end(10)
                .margin_start(10)
                .build();

            // connect to 'clicked' signal of btn
            btn.connect_clicked(|button| {
                if self.tog.is_true() {
                    button.set_label("false!");
                } else {
                    button.set_label("true");
                }
                self.tog.toggle();
            });

            let win = ApplicationWindow::builder()
                .application(&self.app)
                .title("My Gtk App")
                .child(&btn)
                .build();

            // present the window
            win.present();
        };
        activate_signal_handler
    }
}
