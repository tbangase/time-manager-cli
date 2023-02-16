use chrono::{DateTime, Duration, Local, NaiveDateTime, TimeZone, Utc};
use std::fmt::Display;

use derive_getters::Getters;
use serde::Deserialize;

use crate::PayloadResult;

// TODO: Add two field below
// - ( next remain average_time when quit now )
// - ( expected end time )
#[derive(Debug, Getters)]
pub struct WorkingInfo {
    working_time: Duration,
    average_remain_time: Duration,
    minimum_average_remain_time: Duration,
    remain_time: Duration,
    rested_time: Duration,
    estimeted_end_time: DateTime<Local>,
    status: WorkingStatus,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum WorkingStatus {
    NotWorking,
    Working,
    Breaking,
    Finished,
}

impl From<PayloadResult> for WorkingInfo {
    fn from(payload: PayloadResult) -> Self {
        let working_time = Duration::seconds((*payload.working_hours() * 60. * 60.) as i64);
        let average_remain_time =
            Duration::seconds((*payload.average_remain_hours() * 60. * 60.) as i64);
        let minimum_average_remain_time =
            Duration::seconds((*payload.minimum_average_remain_hours() * 60. * 60.) as i64);

        let current = Local::now();
        let rest_timestamp = if let Some(v) = *payload.rest_timestamp() {
            if v != 0 {
                NaiveDateTime::from_timestamp_opt(v / 1000, 0)
            } else {
                None
            }
        } else {
            None
        };

        let resting_time = if let Some(v) = rest_timestamp {
            let rest_started_time = Utc.from_local_datetime(&v).unwrap();
            current - rest_started_time.with_timezone(&Local)
        } else {
            Duration::seconds(0)
        };

        let rested_time =
            Duration::seconds((*payload.rested_hours() * 60. * 60.) as i64) + resting_time;

        // tracing::info!(
        //     "
        // current: {:?}
        // fetched rest_timestamp: {:?}
        // rested_hours: {:?}
        // resting_time: {:?}",
        //     current,
        //     rest_timestamp,
        //     payload.rested_hours(),
        //     resting_time
        // );

        Self {
            working_time,
            average_remain_time,
            minimum_average_remain_time,
            rested_time,
            remain_time: average_remain_time - working_time,
            estimeted_end_time: Local::now() + average_remain_time - working_time,
            status: *payload.status(),
        }
    }
}

impl Display for WorkingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let working_time = self.working_time();
        let average_remain_time = self.average_remain_time();
        let remain_time = self.remain_time();
        let minimum_average_remain_time = self.minimum_average_remain_time();
        writeln!(f, " Your working status ↓ =======================")?;
        writeln!(f, " |  Status:             {:?}", self.status())?;
        writeln!(
            f,
            " |  Working time                              :  {:0>2}:{:0>2}",
            working_time.num_hours(),
            working_time.num_minutes() % 60
        )?;
        writeln!(
            f,
            " |  Rested time                               :  {:0>2}:{:0>2}",
            self.rested_time().num_hours(),
            self.rested_time().num_minutes() % 60
        )?;
        writeln!(
            f,
            " |  Average remain time in this month         :  {:0>2}:{:0>2}",
            average_remain_time.num_hours(),
            average_remain_time.num_minutes() % 60
        )?;
        writeln!(
            f,
            " |  Minimum average remain time in this month :  {:0>2}:{:0>2}",
            minimum_average_remain_time.num_hours(),
            minimum_average_remain_time.num_minutes() % 60
        )?;
        writeln!(
            f,
            " |  Remain time                               : {}{:0>2}:{:0>2}",
            if remain_time.num_seconds() < 0 {
                "-"
            } else {
                " "
            },
            remain_time.num_hours(),
            (remain_time.num_minutes() % 60).abs()
        )?;
        writeln!(
            f,
            " |  Estimated finished time today             :  {}",
            self.estimeted_end_time()
        )?;
        writeln!(f, " | ============================================")
    }
}
