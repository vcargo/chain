use anyhow::anyhow;
use chrono::{Datelike, Duration, Utc, Weekday};
use chrono_tz::Tz;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum MHKind {
    Open,
    Closed,
    Range,
}

#[derive(Debug)]
pub struct WeeklySchedule {
    pub timezone: String,
    pub weekday: Weekday,
    pub mh_kind: MHKind,
    pub open_time: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceFeed {
    pub id: String,
    pub attributes: HashMap<String, String>,
}

impl PriceFeed {
    // https://benchmarks.pyth.network/docs
    pub async fn get_all() -> Result<Vec<PriceFeed>, anyhow::Error> {
        let url = "https://benchmarks.pyth.network/v1/price_feeds/";
        let price_feeds = reqwest::get(url).await?.json::<Vec<PriceFeed>>().await?;
        Ok(price_feeds)
    }

    // https://docs.pyth.network/price-feeds/pythnet-reference/schedule-format
    pub async fn parse(&self) -> Result<Vec<WeeklySchedule>, anyhow::Error> {
        if let Some(schedule) = self.attributes.get("schedule") {
            let parts: Vec<&str> = schedule.split(';').collect();
            if parts.len() != 3 {
                return Err(anyhow!("invalid schedule: {}", schedule));
            }

            let timezone = parts[0].trim();
            let tz: Tz = timezone.parse().map_err(|e| anyhow!("{}", e))?;
            let date = Utc::now().with_timezone(&tz);

            let mut holidays = HashMap::new();
            for pair in parts[2].trim().split(',') {
                let mut kv = pair.split('/');
                if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                    holidays.insert(key.to_string(), value.to_string());
                }
            }

            let res: Result<Vec<WeeklySchedule>, Box<dyn Error>> = parts[1]
                .trim()
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .enumerate()
                .map(|(i, &part)| {
                    let mut part = part;
                    let days = (7 - date.weekday().num_days_from_monday() + i as u32) % 7;
                    let date = date + Duration::days(days as i64);
                    let date = date.format("%m%d").to_string();
                    if let Some(value) = holidays.get(&date) {
                        part = value;
                    }

                    let (mh_kind, open_time) = match part {
                        "O" => (MHKind::Open, "00:00-24:00".to_string()),
                        "C" => (MHKind::Closed, "".to_string()),
                        _ => {
                            let parts: Vec<&str> = part.split('&').collect();
                            let parts = parts
                                .iter()
                                .map(|&part| {
                                    let ranges: Vec<&str> = part.split('-').collect();
                                    format!(
                                        "{}:{}-{}:{}",
                                        &ranges[0][0..2],
                                        &ranges[0][2..4],
                                        &ranges[1][0..2],
                                        &ranges[1][2..4]
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(",");
                            (MHKind::Range, parts)
                        }
                    };

                    Ok(WeeklySchedule {
                        timezone: timezone.to_string(),
                        weekday: Weekday::try_from((i as u8 + 1) % 7)?,
                        mh_kind,
                        open_time,
                    })
                })
                .collect();
            match res {
                Ok(sessions) => Ok(sessions),
                Err(e) => Err(anyhow!("{}", e)),
            }
        } else if let Some(weekly_schedule) = self.attributes.get("weekly_schedule") {
            let parts: Vec<&str> = weekly_schedule.split(',').collect();
            if parts.len() != 8 {
                return Err(anyhow!("invalid weekly_schedule: {}", weekly_schedule));
            }

            let timezone = parts[0].trim();

            let res: Result<Vec<WeeklySchedule>, Box<dyn Error>> = parts
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, &part)| {
                    let (mh_kind, open_time) = match part {
                        "O" => (MHKind::Open, "00:00-24:00"),
                        "C" => (MHKind::Closed, ""),
                        _ => (MHKind::Range, part),
                    };

                    let w = i as u8;

                    Ok(WeeklySchedule {
                        timezone: timezone.to_string(),
                        weekday: Weekday::try_from(w % 7)?,
                        mh_kind,
                        open_time: open_time.to_string(),
                    })
                })
                .collect();
            match res {
                Ok(sessions) => Ok(sessions),
                Err(e) => Err(anyhow!("{}", e)),
            }
        } else {
            let timezone = "America/New_York";
            let open_time = "00:00-24:00";
            let res: Result<Vec<WeeklySchedule>, Box<dyn Error>> = (0..7)
                .map(|w| {
                    let weekday = Weekday::try_from((w + 1) % 7)?;
                    Ok(WeeklySchedule {
                        timezone: timezone.to_string(),
                        weekday,
                        mh_kind: MHKind::Open,
                        open_time: open_time.to_string(),
                    })
                })
                .collect();
            match res {
                Ok(sessions) => Ok(sessions),
                Err(e) => Err(anyhow!("{}", e)),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let price_feeds = PriceFeed::get_all().await?;
    for price_feed in price_feeds {
        let weekly_schedules = price_feed.parse().await?;
        println!("{:?}", price_feed);
        println!("{:#?}", weekly_schedules);
        println!();
    }
    Ok(())
}
