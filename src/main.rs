use gpui::*;
use settings::{default_settings, Settings, SettingsStore};
use theme::ThemeSettings;
use ui::{prelude::*, Tooltip};

struct HelloWorld {
    text: SharedString,
}

impl HelloWorld {
    pub fn render_sign_in_button(&mut self, _: &mut ViewContext<Self>) -> Button {
        Button::new("button_id", "Test!")
            .tooltip(|cx| Tooltip::text("Sign in with Vercel", cx))
            .label_size(LabelSize::Large)
            .on_click(|event, cx| println!("hi"))
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(self.render_sign_in_button(_cx))
    }
}

pub fn init(cx: &mut AppContext) {
    let mut settings = SettingsStore::new(cx);
    settings
        .set_default_settings(&default_settings(), cx)
        .unwrap();
    cx.set_global(settings);

    theme::init(theme::LoadThemes::All(Box::new(assets::Assets)), cx);
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        init(cx);

        cx.activate(true);
        cx.on_action(quit);
        cx.on_action(logout_vercel);
        cx.set_menus(vec![
            Menu {
                name: "set_menus".into(),
                items: vec![
                    MenuItem::action("Quit", Quit),
                    MenuItem::action("Quit 2", Quit),
                    MenuItem::action("Log out of Vercel", LogoutVercel),
                ],
            },
            Menu {
                name: "authorization".into(),
                items: vec![
                    MenuItem::action("Log out of Vercel", LogoutVercel),
                    MenuItem::action("Log out of GitHub", LogoutGithub),
                    MenuItem::separator(),
                    MenuItem::action("Log out of everything", LogoutAll),
                ],
            },
        ]);

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .expect("an err");
    });
}

actions!(authorization, [LogoutVercel]);
actions!(authorization, [LogoutGithub]);
actions!(authorization, [LogoutAll]);

fn logout_vercel(_: &LogoutVercel, cx: &mut AppContext) {
    println!("Logging out of Vercel");
}
fn logout_github(_: &LogoutGithub, cx: &mut AppContext) {
    println!("Logging out of GitHub");
}
fn logout_all(_: &LogoutAll, cx: &mut AppContext) {
    println!("Logging out of everything");
}

actions!(set_menus, [Quit]);

fn quit(_: &Quit, cx: &mut AppContext) {
    println!("Gracefully quitting");
    cx.quit();
}
