use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use crate::windows::main_window::MainWindow;

    use super::*;

    #[derive(Debug, Default)]
    pub struct BakeryOSUpdateApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for BakeryOSUpdateApplication {
        const NAME: &'static str = "BakeryOSUpdateApplication";
        type Type = super::BakeryOSUpdateApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for BakeryOSUpdateApplication {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl ApplicationImpl for BakeryOSUpdateApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = MainWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for BakeryOSUpdateApplication {}
    impl AdwApplicationImpl for BakeryOSUpdateApplication {}
}

glib::wrapper! {
    pub struct BakeryOSUpdateApplication(ObjectSubclass<imp::BakeryOSUpdateApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl BakeryOSUpdateApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/org/bakeryos/update")
            .build()
    }
}
