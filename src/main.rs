extern crate hiirc;
extern crate time;

use hiirc::*;
use time::Duration;

struct Zedonius {
  blah: bool,
}

const NICKNAME: &'static str = "zedonius";
const USERNAME: &'static str = "hiirc";
const REALNAME: &'static str = "Ruler of Flame";

impl Listener for Zedonius {
    fn any(&mut self, irc: &Irc, event: &Event) {
        println!("{:?}", event);
    }

    fn channel_msg(&mut self, irc: &Irc, channel: &Channel, user: &ChannelUser, msg: &str) {
        if msg.starts_with(NICKNAME) {
            println!("got highlighted");
        }
    }

    fn reconnect(&mut self, irc: &Irc) {
        println!("reconnect");
    }

    fn welcome(&mut self, irc: &Irc) {
        irc.join("#clux", None);
    }
}

fn main() {
    Settings::new("irc.freenode.net:6667", NICKNAME)
        .username(USERNAME)
        .realname(REALNAME)
        .reconnection(ReconnectionSettings::Reconnect {
            max_attempts: 0,
            delay_between_attempts: Duration::seconds(5),
            delay_after_disconnect: Duration::seconds(15),
        })
        .dispatch(Zedonius{ blah: true }).unwrap();
}
