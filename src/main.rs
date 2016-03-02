#![allow(unused_variables)]
extern crate hiirc;
extern crate time;
use std::thread;
use std::sync::Arc;

use hiirc::*;
use time::Duration;

struct Zedonius {
  blah: u32,
}

const NICKNAME: &'static str = "zedonius";
const USERNAME: &'static str = "hiirc";
const REALNAME: &'static str = "Ruler of Flame";

impl Listener for Zedonius {
    fn any(&mut self, irc: Arc<Irc>, event: &Event) {
        println!("{:?}", event);
    }

    fn channel_msg(&mut self, irc: Arc<Irc>, channel: Arc<Channel>, user: Arc<ChannelUser>, msg: &str) {
        if msg.starts_with(NICKNAME) {
            self.blah += 1;
            let resp = format!("blah {}", self.blah);
            irc.privmsg("#clux", &resp); // channel.name is private so can't actually use it atm
            thread::spawn(move || {
                thread::sleep(std::time::Duration::from_secs(4));
                irc.privmsg("#clux", "done");
            });
        }
    }

    fn reconnect(&mut self, irc: Arc<Irc>) {
        println!("reconnect");
    }

    fn welcome(&mut self, irc: Arc<Irc>) {
        irc.join("#clux", None);
    }
}

fn main() {
    Settings::new("irc.freenode.net:6667", NICKNAME)
        .username(USERNAME)
        .realname(REALNAME)
        .reconnection(ReconnectionSettings::Reconnect {
            max_attempts: 0,
            delay_between_attempts: Duration::seconds(15),
            delay_after_disconnect: Duration::seconds(30),
        })
        .dispatch(Zedonius{ blah: 0 }).unwrap();
}
