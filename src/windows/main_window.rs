use adw::prelude::{ActionRowExt, PreferencesGroupExt, PreferencesRowExt};
use adw::subclass::prelude::*;
use async_channel::{Receiver, Sender};
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::helpers::{get_update_avaliable_package, update_package};
use crate::models::event::Event;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/bakeryos/update/main_window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub status_label: TemplateChild<gtk::Label>,

        #[template_child]
        pub update_btn: TemplateChild<gtk::Button>,

        #[template_child]
        pub package_list: TemplateChild<adw::PreferencesGroup>,

        pub selected_packages: RefCell<Vec<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {}
    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl MainWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let obj: MainWindow = glib::Object::builder()
            .property("application", application)
            .build();

        let imp = obj.imp();
        let (sender, receiver) = async_channel::unbounded::<Event>();
        imp.handle_event(&sender, receiver);
        imp.load_data(&sender);

        let sender = sender.clone();
        imp.update_btn.connect_clicked(clone!(
            #[weak]
            obj,
            move |_| {
                obj.imp().on_update_btn_clicked(&sender);
            }
        ));

        obj
    }
}

impl imp::MainWindow {
    pub fn load_data(&self, sender: &Sender<Event>) {
        let sender = sender.clone();

        gio::spawn_blocking(move || {
            let result = get_update_avaliable_package();
            match result {
                Ok(packages) => {
                    let _ = sender.send_blocking(Event::PackageLoadedSuccess(packages));
                }
                Err(e) => {
                    let _ = sender.send_blocking(Event::PackageLoadedFailed(e));
                }
            }
        });
    }

    pub fn handle_event(&self, sender: &Sender<Event>, receiver: Receiver<Event>) {
        let sender = sender.clone();
        glib::spawn_future_local(clone!(
            #[weak(rename_to=t)]
            self,
            async move {
                while let Ok(event) = receiver.recv().await {
                    match event {
                        Event::PackageLoadedSuccess(p) => {
                            t.reset_select_package();
                            t.main_stack.set_visible_child_name("package_show_layout");
                            t.show_package_list(p);
                        }

                        Event::PackageLoadedFailed(_) => {
                            t.main_stack.set_visible_child_name("update_failed_layout");
                        }

                        Event::PackageUpdate => {
                            t.main_stack.set_visible_child_name("update_layout");
                            t.spawn_update_task(&sender);
                        }
                        Event::PackageUpdateSuccess => {
                            t.main_stack.set_visible_child_name("update_success_layout");
                        }
                        Event::PackageUpdateFailed(_) => {
                            t.main_stack.set_visible_child_name("update_failed_layout");
                        }
                    }
                }
            }
        ));
    }

    pub fn spawn_update_task(&self, sender: &Sender<Event>) {
        let sender = sender.clone();
        let packages = self.selected_packages.borrow().clone();

        gio::spawn_blocking(move || {
            let result = update_package(packages);
            match result {
                Ok(()) => {
                    let _ = sender.send_blocking(Event::PackageUpdateSuccess);
                }
                Err(e) => {
                    let _ = sender.send_blocking(Event::PackageUpdateFailed(e));
                }
            }
        });
    }

    pub fn toggle_select_package(&self, name: &str) {
        let mut selected_packages = self.selected_packages.borrow_mut();
        if let Some(pos) = selected_packages.iter().position(|pkg| pkg == name) {
            selected_packages.remove(pos);
        } else {
            selected_packages.push(name.to_string());
        }
    }

    pub fn reset_select_package(&self) {
        let mut selected_packages = self.selected_packages.borrow_mut();
        *selected_packages = vec![];
    }

    pub fn show_package_list(&self, packages: Vec<String>) {
        for package in packages.iter() {
            self.toggle_select_package(package);
            let row = self.build_list_row(package);
            self.package_list.add(&row);
        }
    }

    pub fn build_list_row(&self, package_name: &str) -> adw::ActionRow {
        let row = adw::ActionRow::new();
        let check_btn = gtk::CheckButton::new();
        check_btn.set_valign(gtk::Align::Center);
        check_btn.set_active(true);

        let package_name_string = package_name.to_string();

        check_btn.connect_toggled(clone!(
            #[weak(rename_to=t)]
            self,
            move |_| {
                t.toggle_select_package(&package_name_string);
            }
        ));

        row.set_title(package_name);
        row.add_suffix(&check_btn);
        row.set_activatable_widget(Some(&check_btn));

        row
    }

    pub fn on_update_btn_clicked(&self, sender: &Sender<Event>) {
        let sender = sender.clone();
        glib::spawn_future_local(clone!(async move {
            let _ = sender.send(Event::PackageUpdate).await;
        }));
    }
}
