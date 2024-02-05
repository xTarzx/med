use chrono::{NaiveDate, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, Clone, Copy, Display)]
pub enum MediaType {
    Anime,
    TVShow,
    Game,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumIter, Clone, Copy, Display)]
pub enum Status {
    OnGoing,
    Finished,
    Upcoming,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Media {
    pub title: String,
    pub alt_titles: Vec<String>,
    pub media_type: MediaType,
    pub release_date: NaiveDate,
    pub status: Status,
    pub broadcast_day: Option<Weekday>,
    pub broadcast_time: Option<NaiveTime>,
    pub current: Option<u32>,
    pub total: Option<u32>,
    pub total_out: Option<u32>,
}

impl Default for Media {
    fn default() -> Self {
        Media {
            title: String::new(),
            alt_titles: Vec::new(),
            media_type: MediaType::Anime,
            release_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            status: Status::Finished,
            broadcast_day: None,
            broadcast_time: None,
            current: None,
            total: None,
            total_out: None,
        }
    }
}
