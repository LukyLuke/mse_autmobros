extern crate librobotcontrol_sys;

use std::thread::sleep;
use std::time::Duration;

fn main() {
	let led = librobotcontrol_sys::RC_LED_RED;
	unsafe {
		librobotcontrol_sys::rc_led_blink(led, 10.0, 10.0);
		librobotcontrol_sys::rc_led_stop_blink(led);
	}
}
