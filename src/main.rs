use gtk::{
    gio::{Cancellable, Notification},
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{ButtonExt, GtkWindowExt},
    Application, ApplicationWindow, Button,
};

const APP_ID: &str = "org.gnome.Console";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    match app.register(None::<&Cancellable>) {
        Ok(()) => println!("Registration succeeded"),
        Err(error) => eprintln!("Registration failed: {}", error),
    }
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let notification = Notification::new("hello world");
    notification.set_body(Some("notif test"));

    let button = Button::builder()
        .label("Click me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let apps = app.clone();
    button.connect_clicked(move |_| {
        apps.send_notification(Some(APP_ID), &notification);
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Notification Example")
        .child(&button)
        .build();
    window.present();
}
