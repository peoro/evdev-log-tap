#![allow(non_snake_case)]

extern crate evdev;

use std::time::{Duration, SystemTime};


fn asMillis( duration: Duration ) -> u64 {
	duration.as_secs()*1000 + duration.subsec_millis() as u64
}

fn main() {
	let mut args = std::env::args_os();
	let mut d = evdev::Device::open(&args.nth(1).unwrap()).unwrap();

	//println!("{}", d);
	//println!("Events:");

	let mut pressed = false;
	let start = SystemTime::now();
	let mut prevEvent = start;
	let mut prevPress = start;

	let mut toggle = |pressed: &mut bool| {
		let now = SystemTime::now();

		let time = now.duration_since( start ).unwrap();
		let relTime = now.duration_since( prevEvent ).unwrap();

		prevEvent = now;
		*pressed = ! *pressed;

		if *pressed {
			println!(
				"▣ {:02}.{:03}  +{:4}",
				time.as_secs(), time.subsec_millis(),
				asMillis( relTime )
			);
			prevPress = now;
		} else {
			println!(
				"▢ {:02}.{:03}  +{:4}  ({:4})",
				time.as_secs(), time.subsec_millis(),
				asMillis( relTime ),
				asMillis( now.duration_since(prevPress).unwrap() )
			);
		}
	};

	loop {
		for ev in d.events_no_sync().unwrap() {
			if ev.code == evdev::ABS_PRESSURE.number() {
				//println!("0x{:x} 0x{:x}", ev._type, ev.code);
				// println!("{:?}", ev);
				if pressed && ev.value == 0 {
					toggle( &mut pressed );
				}
				else if ! pressed {
					assert!( ev.value > 0 );
					toggle( &mut pressed );
				}
			}
		}
	}
}
