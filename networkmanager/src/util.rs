// SPDX-License-Identifier: MPL-2.0

use jiff::Timestamp;

pub fn clock_boottime_to_time(time: i32) -> Option<Timestamp> {
	let boot_time = procfs::boot_time_secs()
		.ok()
		.and_then(|boot_time| i64::try_from(boot_time).ok())?;
	Timestamp::from_second(boot_time + time as i64).ok()
}
