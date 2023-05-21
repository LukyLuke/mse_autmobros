use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod librobotcontrol;

fn main() {
	// Pause and Start buttons forn controlling
	librobotcontrol::init_button(librobotcontrol::Button::Pause);
	librobotcontrol::init_button(librobotcontrol::Button::Mode);
	librobotcontrol::register_button_callback(librobotcontrol::Button::Pause, pause_pressed);
	librobotcontrol::register_button_callback(librobotcontrol::Button::Mode, mode_pressed);

	// Encoders and Motors
	librobotcontrol::init_encoders();

	let terminate = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate)).unwrap();
	while !terminate.load(Ordering::Relaxed) {
		sleep(Duration::from_millis(100));

		let enc1 = librobotcontrol::get_encoder_value(1);
		let enc2 = librobotcontrol::get_encoder_value(2);
		let enc3 = librobotcontrol::get_encoder_value(3);
		let enc4 = librobotcontrol::get_encoder_value(4);

		println!("Encoders:\t{}\t{}\t{}\t{}", enc1, enc2, enc3, enc4);
	}
	librobotcontrol::cleanup();
}

extern "C" fn pause_pressed() {
	println!("Pause Pressed!");
}

extern "C" fn mode_pressed() {
	println!("Mode Pressed!");
}
