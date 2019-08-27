use mammut::{Mastodon, Registration};
use mammut::apps::{AppBuilder, Scopes};
use mammut::status_builder::{StatusBuilder, Visibility};
use super::error::Result;

pub fn register() -> Result<Mastodon> {
    let app = AppBuilder {
        client_name: "SabaToot",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::Write,
        website: Some("https://github.com/inux39/sabatoot"),
    };
    let base = {
        let mut s = String::new();
        println!("Instance URL:");
        std::io::stdin().read_line(&mut s)?;
        if !s.starts_with("https") {
            s = format!("https://{}", s);
        }
        s.trim().to_string()
    };

    let mut registration = Registration::new(base);
    registration.register(app)?;
    let url = registration.authorise()?;

    println!("Authorize URL: {}", url);
    println!("returned code: ");

    let code = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
        s.trim().to_string()
    };

    let mastodon = registration.create_access_token(code.to_string())?;
    Ok(mastodon)
}

pub fn toot(mastodon: Mastodon, v: Option<Visibility>, txt: String, cw_txt: Option<String>) -> Result<()> {
    let mut status = StatusBuilder::new(txt);
    status.visibility = v;
    status.spoiler_text = cw_txt;
    mastodon.new_status(status)?;
    Ok(())
}

