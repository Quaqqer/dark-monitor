#![feature(async_closure)]

mod daemon;

use daemon::start_daemon;

const DESKTOP_DESTINATION: &str = "org.freedesktop.portal.Desktop";
const DESKTOP_PATH: &str = "/org/freedesktop/portal/desktop";

#[tokio::main]
async fn main() {
    start_daemon();
    // let (color_scheme,): (Variant<Box<dyn RefArg>>,) = proxy
    //     .method_call(
    //         "org.freedesktop.portal.Settings",
    //         "Read",
    //         ("org.freedesktop.appearance", "color-scheme"),
    //     )
    //     .unwrap();
    //
    // let x: u64 = (*color_scheme.0).as_u64().unwrap();
    // println!("{}", x);
}

enum ColorSchemePreference {
    Default = 0,
    PreferDark = 1,
    PreferLight = 2,
}

impl TryFrom<u64> for ColorSchemePreference {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorSchemePreference::Default),
            1 => Ok(ColorSchemePreference::PreferDark),
            2 => Ok(ColorSchemePreference::PreferLight),
            _ => Err("Could not convert value to color scheme preference"),
        }
    }
}

trait ColorSchemeInterface {
    // fn add_listener(&mut self, listener: impl Fn(ColorSchemePreference) -> ()) {}
    //
    // async fn listen(&self);
}

async fn daemon() {
    // let conn = Connection::new_session().unwrap();
    //
    // let proxy = conn.with_proxy(DESKTOP_DESTINATION, DESKTOP_PATH, Duration::from_secs(1));
    //
    // let t = proxy
    //     .match_start(
    //         MatchRule::new()
    //             .with_member("SettingChanged")
    //             .with_path(DESKTOP_PATH),
    //         true,
    //         Box::new(|msg, _conn| {
    //             if msg.read2().unwrap() == ("org.freedesktop.appearance", "color-scheme") {
    //                 let (x, y, z): (&str, &str, Variant<Box<dyn RefArg>>) = msg.read3().unwrap();
    //                 println!("{:?}", msg);
    //                 println!("Property {} {} {}", x, y, z.as_u64().unwrap());
    //             }
    //             true
    //         }),
    //     )
    //     .unwrap();
    //
    // loop {
    //     conn.process(Duration::from_secs(1)).unwrap();
    // }
    //
    // proxy.match_stop(t, true).unwrap();
}

async fn set_color_scheme() {}

async fn get_color_scheme() {}

async fn toggle_color_scheme() {}
