#![allow(unused_variables)]
extern crate hiirc;
extern crate time;

#[macro_use] extern crate log;
extern crate loggerv;

use std::thread;
use std::sync::Arc;

use hiirc::*;
use time::Duration;

struct Zedonius {
    request_count: u32,
    nickname: &'static str,
}

#[allow(unused_must_use)]
impl Listener for Zedonius {
    fn any(&mut self, irc: Arc<Irc>, event: &Event) {
        debug!("{:?}", event);
    }

    fn channel_msg(&mut self,
                   irc: Arc<Irc>,
                   channel: Arc<Channel>,
                   user: Arc<ChannelUser>,
                   msg: &str) {
        if msg.starts_with(self.nickname) {
            info!("{}: {}", &user.nickname(), &msg);
            self.request_count += 1;
            let cpy = self.request_count;
            thread::spawn(move || {
                thread::sleep(std::time::Duration::from_secs(4));
                let resp = format!("request_count {}", cpy);
                irc.privmsg(channel.name(), &resp);
            });
        }
    }

    fn reconnect(&mut self, irc: Arc<Irc>) {
        warn!("Reconnect");
    }

    fn welcome(&mut self, irc: Arc<Irc>) {
        info!("Connected");
        irc.join("#clux", None);
    }
}

fn main() {
    // NB: will fail to start if nickname is not unique
    let ziddy = Zedonius { nickname: "zedonius", request_count: 0 };
    loggerv::init_with_verbosity(1).unwrap();
    Settings::new("irc.quakenet.org:6667", &ziddy.nickname)
        .username("ziddy")
        .realname("Ruler of Flame")
        .reconnection(ReconnectionSettings::Reconnect {
            max_attempts: 0,
            delay_between_attempts: Duration::seconds(15),
            delay_after_disconnect: Duration::seconds(30),
        })
        .dispatch(ziddy)
        .unwrap();
}
