// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use chrono::TimeZone;

const TZ_FORMAT: &str = "%a %Y-%m-%d %H:%M:%S %Z";
const RTC_FORMAT: &str = "%a %Y-%m-%d %H:%M:%S";
const CHOICES: &[&str] = &["no", "yes"];

#[tokio::main]
pub async fn main() -> zbus::Result<()> {
	let connection = zbus::Connection::system().await?;
	let proxy = timedate_zbus::TimeDateProxy::new(&connection).await?;

	let ntp_service = if proxy.ntp().await? {
		"active"
	} else {
		"inactive"
	};

	let rtc_in_local = proxy.local_rtc().await?;
	let rtc_time_usecs = proxy.rtctime_usec().await?;
	let time_usecs = proxy.time_usec().await?;
	let timezone = proxy.timezone().await?;

	let tz: chrono_tz::Tz = timezone.parse().unwrap();

	let datetime = tz.timestamp_millis_opt((time_usecs / 1000) as i64).unwrap();

	let rtc_millis = (rtc_time_usecs / 1000) as i64;
	let rtc_time = (if rtc_in_local {
		tz.timestamp_millis_opt(rtc_millis).unwrap()
	} else {
		chrono_tz::UTC.timestamp_millis_opt(rtc_millis).unwrap()
	})
	.format(RTC_FORMAT);

	let local = datetime.format(TZ_FORMAT);
	let universal = datetime.with_timezone(&chrono_tz::UTC).format(TZ_FORMAT);
	let tz_string = datetime.format("%Z, %z");

	let rtc_in_local = CHOICES[usize::from(rtc_in_local)];
	let synchronized = CHOICES[usize::from(proxy.ntp_synchronized().await.unwrap_or_default())];

	println!(
		"               Local time: {local}
           Universal time: {universal}
                 RTC time: {rtc_time}
                Time zone: {timezone} ({tz_string})
System clock synchronized: {synchronized}
              NTP Service: {ntp_service}
          RTC in local TZ: {rtc_in_local}"
	);

	Ok(())
}
