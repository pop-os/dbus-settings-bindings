// SPDX-License-Identifier: MPL-2.0

use time::OffsetDateTime;

pub fn clock_boottime_to_time(time: i32) -> Option<OffsetDateTime> {
	let boot_time = procfs::boot_time_secs()
		.ok()
		.and_then(|boot_time| i64::try_from(boot_time).ok())?;
	OffsetDateTime::from_unix_timestamp(boot_time + time as i64).ok()
}
