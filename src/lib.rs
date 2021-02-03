use std::{
    io::{self, Error, ErrorKind},
    time::Duration,
};
use weechat::{
    buffer::Buffer,
    config::{Config, ConfigSectionSettings, StringOptionSettings},
    hooks::{BarItem, RemainingCalls, TimerHook},
    plugin, Args, Plugin, Weechat,
};

struct BadIdea {
    _timer: TimerHook,
    _config: Config,
}

fn get_front(url: &str) -> io::Result<String> {
    Ok(ureq::get(url)
        .call()
        .map_err(|why| Error::new(ErrorKind::Other, format!("{}", why)))?
        .into_string()?)
}

fn update_front_item(wc: &Weechat, _: &Buffer) -> String {
    if let weechat::config::ConfigOption::String(url) = wc.config_get("badidea.front.url").unwrap()
    {
        match get_front(&url.value()) {
            Ok(who) => who,
            Err(why) => {
                Weechat::print(&format!("badidea::front\tcan't fetch front: {}", why));
                "Mi Error".to_string()
            }
        }
    } else {
        "set badidea.front.url".to_string()
    }
}

impl Plugin for BadIdea {
    fn init(_: &Weechat, _: Args) -> Result<Self, ()> {
        let mut conf = Config::new("badidea").expect("can't make badidea");

        {
            let front_section = ConfigSectionSettings::new("front");

            let mut section = conf.new_section(front_section)?;

            let front_url = StringOptionSettings::new("url")
                .default_value("https://home.cetacean.club/front")
                .description("the URL to hit to check who is front");

            section.new_string_option(front_url)?;
        }

        conf.write().expect("to write config");
        conf.read().expect("to read config");

        let front_item = BarItem::new("front", update_front_item)?;

        let timer = TimerHook::new(
            Duration::from_secs(300),
            0,
            0,
            move |_: &Weechat, _: RemainingCalls| {
                front_item.update();
            },
        )?;

        let result = Self {
            _timer: timer,
            _config: conf,
        };

        Ok(result)
    }
}

impl Drop for BadIdea {
    fn drop(&mut self) {}
}

plugin!(
    BadIdea,
    name: "badidea",
    author: "Cadey A. Ratio <cadey@firemail.cc>",
    description: "Integrate Weechat into my software-driven insanity",
    version: "0.1.0",
    license: "WTFPL"
);
