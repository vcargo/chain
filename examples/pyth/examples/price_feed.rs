use anyhow::anyhow;
use chrono::{NaiveTime, Weekday};
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
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
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

    // https://docs.pyth.network/price-feeds/how-pyth-works/weekly-schedule-format
    pub async fn parse(&self) -> Result<Vec<WeeklySchedule>, anyhow::Error> {
        let weekly_schedule = match self.attributes.get("weekly_schedule") {
            Some(s) => s,
            None => {
                return Ok(vec![]);
            }
        };

        let parts: Vec<&str> = weekly_schedule.split(",").collect();
        if parts.len() != 8 {
            return Err(anyhow!("invalid weekly_schedule: {}", weekly_schedule));
        }

        let timezone = parts[0].trim();
        timezone
            .parse::<Tz>()
            .map_err(|e| anyhow!("timezone error: {:?}", e))?;

        let res: Result<Vec<WeeklySchedule>, Box<dyn Error>> = parts
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &part)| {
                let (mh_kind, start, end) = match part {
                    "O" => (MHKind::Open, "00:00", "00:00"),
                    "C" => (MHKind::Closed, "00:00", "00:00"),
                    _ => {
                        let (start, mut end) = part
                            .split_once("-")
                            .ok_or(format!("'{}' is not a valid time range", part))?;
                        if end == "24:00" {
                            end = "00:00";
                        }
                        (MHKind::Range, start, end)
                    }
                };

                let w = i as u8;

                Ok(WeeklySchedule {
                    timezone: timezone.to_string(),
                    weekday: Weekday::try_from(w % 7)?,
                    mh_kind,
                    start_time: NaiveTime::parse_from_str(start, "%H:%M")?,
                    end_time: NaiveTime::parse_from_str(end, "%H:%M")?,
                })
            })
            .collect();

        match res {
            Ok(sessions) => Ok(sessions),
            Err(e) => Err(anyhow!("{}", e)),
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
