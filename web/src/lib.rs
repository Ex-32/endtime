pub use chrono;

use chrono::{DateTime, TimeDelta, TimeZone, Utc};
use wasm_bindgen::prelude::*;

pub const ENDTIME_EPOCH: DateTime<Utc> = DateTime::<Utc>::from_timestamp(1356048000, 0)
    .expect("Unable to construct constant UTC timestamp");

pub fn since_endtime<Tz: TimeZone>(datetime: DateTime<Tz>) -> TimeDelta {
    datetime.signed_duration_since(ENDTIME_EPOCH)
}

pub fn endtime_now() -> TimeDelta {
    since_endtime(chrono::Local::now())
}

pub fn endtime_string(
    end_datetime: TimeDelta,
    digits: [char; 12],
    frac_digits: usize,
) -> (String, String) {
    const SECS_PER_DAY: i64 = 86_400;

    let days: i64 = end_datetime.num_days();
    let frac_day: f64 = (((end_datetime.num_seconds() - (days * SECS_PER_DAY)) as f64)
        / SECS_PER_DAY as f64)
        + ((end_datetime.subsec_nanos() as f64) / (1_000_000_000 * SECS_PER_DAY) as f64);

    dbg!(&days);
    dbg!(&frac_day);

    let int_part = match days {
        0 => "0".to_owned(),
        _ => {
            let mut days_pt: u64 = i64::abs(days) as u64;
            let mut ret = vec![];
            while days_pt > 0 {
                ret.push(digits[(days_pt % 12) as usize]);
                days_pt /= 12;
            }
            if days < 0 {
                ret.push('-');
            }
            ret.into_iter().rev().collect::<String>()
        }
    };

    let frac_part = {
        let mut frac_pt = f64::abs(frac_day);
        let mut ret = vec![];

        for _ in 0..frac_digits {
            frac_pt *= 12.0;
            let digit = frac_pt as usize;
            ret.push(digits[digit]);
            frac_pt = (digit as f64) - 12.0;

            if frac_pt == 0.0 {
                break;
            }
        }

        ret.into_iter().collect::<String>()
    };

    dbg!((int_part, frac_part))
}

#[wasm_bindgen]
pub fn endtime_now_string(sep: &str, dozenals: usize) -> String {
    const DIGITS: [char; 12] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X', 'E'];

    let (mut a, b): (String, String) = endtime_string(endtime_now(), DIGITS, dozenals);

    a.push_str(sep);
    a.push_str(&b);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_since_epoch_is_zero() {
        let delta = since_endtime(ENDTIME_EPOCH.clone());
        assert!(delta.is_zero())
    }
}
