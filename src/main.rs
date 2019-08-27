use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::error::Error;
use clap::{Arg, App};
use mammut::{Mastodon, Data};
use mammut::status_builder::Visibility;

mod error;
mod mastodon;
use error::Result;

const CONFIG_FILE: &'static str = "sabatoot.toml";

fn main() {
    let hash = match git_hash() {
        Ok(o) => o,
        Err(_) => "Unknown".to_string(),
    };
    let clap = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(&*format!("SabaToot {}", hash))
        .arg(Arg::with_name("range")
            .short("r")
            .takes_value(true)
            .possible_values(&["public", "unlisted", "private", "direct"])
            .help("Override account visibility settings"))
        .arg(Arg::with_name("cw")
            .long("cw")
            .takes_value(true)
            .help("Contents Warning"))
        .arg(Arg::with_name("text")
            .takes_value(true)
            .help("Toot text"))
        .get_matches();
    let mastodon = setup();

    let text = match clap.value_of("text") {
        Some(s) => s.to_string(),
        None => {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s
        },
    };
    let visibility = match clap.value_of("range") {
        Some(s) => Some(from(s)),
        None => None,
    };
    let cw = match clap.value_of("cw") {
        Some(s) => Some(s.to_string()),
        None => None,
    };

    let status = mastodon::toot(mastodon, visibility, text, cw);
    match status {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Toot error"),
    };
}

fn setup() -> Mastodon {
    let mut conf = match std::env::current_exe() {
        Ok(o) => o,
        Err(e) => panic!("{}", e.description()),
    };
    conf.pop();
    conf.push(CONFIG_FILE);

    let mastodon = match File::open(&conf) {
        Ok(mut file) => {
            let mut config = String::new();
            // TODO: unwrap()
            match file.read_to_string(&mut config) {
                Ok(f) => f,
                Err(e) => panic!("{}", e.description()),
            };
            let data: Data = match toml::from_str(&config) {
                Ok(o) => o,
                Err(e) => panic!("{}", e.description()),
            };
            Mastodon::from_data(data)
        },
        Err(_) => {
            let config = match mastodon::register() {
                Ok(o) => o,
                Err(_) => return setup(),
            };
            // save keys
            // TODO: unwrap()
            let toml = toml::to_string(&*config).unwrap();
            let mut file = File::create(&conf).unwrap();
            file.write_all(toml.as_bytes()).unwrap();
            config
        },
    };

    mastodon
}

fn from(f: &str) -> Visibility {
    // ["public", "unlisted", "private", "direct"]
    match f {
        "public" => Visibility::Public,
        "unlisted" => Visibility::Unlisted,
        "private" => Visibility::Private,
        "direct" => Visibility::Direct,
        _ => Visibility::Unlisted,
    }
}

fn git_hash() -> Result<String> {
    let git = std::process::Command::new("/usr/bin/git")
        .args(&["rev-parse", "HEAD"])
        .output()?;
    let hash = String::from_utf8(git.stdout)?;

    Ok(hash)
}

