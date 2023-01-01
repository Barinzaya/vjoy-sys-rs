use std::string::{FromUtf16Error};

fn main() {
	unsafe {
		if vjoy_sys::vJoyEnabled() == 0 {
			println!("vJoy Not Available");
			return;
		}

		let (mut interface_version, mut driver_version) = (0u16, 0u16);
		let compatible = vjoy_sys::DriverMatch(&mut interface_version as *mut u16, &mut driver_version as *mut u16) != 0;

		println!("vJoy Interface: v{}.{}.{}", (interface_version >> 8) & 0xf, (interface_version >> 4) & 0xf, (interface_version >> 0) & 0xf);
		println!("vJoy Driver: v{}.{}.{}", (driver_version >> 8) & 0xf, (driver_version >> 4) & 0xf, (driver_version >> 0) & 0xf);

		if !compatible {
			println!("WARNING: Interface and driver version do not match!");
		}

		let product = decode_utf16(vjoy_sys::GetvJoyProductString() as *const u16);
		println!("Product: {}", product.as_ref().map(String::as_ref).unwrap_or("(not valid UTF-16)"));

		let manufacturer = decode_utf16(vjoy_sys::GetvJoyManufacturerString() as *const u16);
		println!("Manufacturer: {}", manufacturer.as_ref().map(String::as_ref).unwrap_or("(not valid UTF-16)"));

		let serial = decode_utf16(vjoy_sys::GetvJoySerialNumberString() as *const u16);
		println!("Serial Number: {}", serial.as_ref().map(String::as_ref).unwrap_or("(not valid UTF-16)"));

		let mut num_devices = 0i32;
		if vjoy_sys::GetNumberExistingVJD(&mut num_devices as *mut i32) == 0 {
			println!("Failed to get number of available vJoy devices!");
			return;
		}

		let mut max_devices = 0i32;
		if vjoy_sys::GetvJoyMaxDevices(&mut max_devices as *mut i32) == 0 {
			println!("Failed to get maximum number of vJoy devices!");
			return;
		}

		println!("Devices: {}/{}", num_devices, max_devices);
		assert_eq!(max_devices, vjoy_sys::VJOY_MAX_N_DEVICES as i32);

		let axes = [
			(vjoy_sys::HID_USAGE_X,   "X"),
			(vjoy_sys::HID_USAGE_Y,   "Y"),
			(vjoy_sys::HID_USAGE_Z,   "Z"),
			(vjoy_sys::HID_USAGE_RX,  "RX"),
			(vjoy_sys::HID_USAGE_RY,  "RY"),
			(vjoy_sys::HID_USAGE_RZ,  "RZ"),
			(vjoy_sys::HID_USAGE_SL0, "Slider"),
			(vjoy_sys::HID_USAGE_SL1, "Dial"),

			(vjoy_sys::HID_USAGE_WHL,         "Wheel"),
			(vjoy_sys::HID_USAGE_ACCELERATOR, "Accelerator"),
			(vjoy_sys::HID_USAGE_BRAKE,       "Brake"),
			(vjoy_sys::HID_USAGE_CLUTCH,      "Clutch"),
			(vjoy_sys::HID_USAGE_STEERING,    "Steering"),
			(vjoy_sys::HID_USAGE_AILERON,     "Aileron"),
			(vjoy_sys::HID_USAGE_RUDDER,      "Rudder"),
			(vjoy_sys::HID_USAGE_THROTTLE,    "Throttle"),
		];

		for device in 1..=max_devices as u32 {
			if vjoy_sys::isVJDExists(device) == 0 {
				continue;
			}

			println!("vJoy Device #{}:", device);
			println!("  Status: {}", match vjoy_sys::GetVJDStatus(device) {
				vjoy_sys::VjdStat_VJD_STAT_OWN  => "Owned",
				vjoy_sys::VjdStat_VJD_STAT_FREE => "Free",
				vjoy_sys::VjdStat_VJD_STAT_BUSY => "Busy",
				vjoy_sys::VjdStat_VJD_STAT_MISS => "Missing",
				_ => "Unknown",
			});

			let num_buttons = vjoy_sys::GetVJDButtonNumber(device);
			println!("  Buttons: {}", num_buttons);

			let num_cpov = vjoy_sys::GetVJDContPovNumber(device);
			let num_dpov = vjoy_sys::GetVJDDiscPovNumber(device);
			println!("  POVs: {} discrete, {} continuous", num_dpov, num_cpov);

			for (i, &(usage, name)) in axes.iter().enumerate() {
				if vjoy_sys::GetVJDAxisExist(device, usage) == 0 {
					continue;
				}

				let mut axis_max = 0i32;
				if vjoy_sys::GetVJDAxisMax(device, usage, &mut axis_max as *mut i32) == 0 {
					continue;
				}

				let mut axis_min = 0i32;
				if vjoy_sys::GetVJDAxisMin(device, usage, &mut axis_min as *mut i32) == 0 {
					continue;
				}

				println!("  Axis #{} ({}): {} to {}", i, name, axis_min, axis_max);
			}
		}
	}
}

unsafe fn decode_utf16(s: *const u16) -> Result<String, FromUtf16Error> {
	let len = (0..).position(|i| s.offset(i).read() == 0).unwrap();
	let slice = std::slice::from_raw_parts(s, len);
	String::from_utf16(slice)
}
