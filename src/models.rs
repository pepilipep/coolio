use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::error::CoolioError;

#[derive(Debug)]
pub struct Listen {
    pub song_id: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    pub automated: bool,
}

#[derive(Debug)]
pub enum ThrowbackPeriod {
    Years(usize),
    Months(usize),
    Weeks(usize),
    Days(usize),
}

impl FromStr for ThrowbackPeriod {
    type Err = CoolioError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_string();
        match s.pop() {
            None => Err("Empty period".into()),
            Some(c) => {
                let per = s
                    .parse::<usize>()
                    .map_err(|_| CoolioError::from("Couldn't parse period"))?;
                if per == 0 || per > 100 {
                    return Err("Period not allowed".into());
                }
                match c {
                    'y' => Ok(ThrowbackPeriod::Years(per)),
                    'm' => Ok(ThrowbackPeriod::Months(per)),
                    'w' => Ok(ThrowbackPeriod::Weeks(per)),
                    'd' => Ok(ThrowbackPeriod::Days(per)),
                    _ => Err(format!("Unknown period {}", c).into()),
                }
            }
        }
    }
}
