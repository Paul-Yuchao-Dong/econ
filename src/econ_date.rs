use std::fmt;

use chrono::{DateTime, Local, Duration, Weekday};
use chrono::Datelike;

// build a struct of nearest Saturday
#[derive(PartialEq, Eq)]
pub struct EconDate {
    dt: DateTime<Local>,
    year: i32,
    issue: i64,
    pub file_name: String
}

impl EconDate {
    pub fn new(dt: DateTime<Local>) -> Self {
        // dbg!(dt);
        let tmp = get_econ_date(dt);
        // dbg!(tmp);
        let issue = {
            let previous = "2023-01-07T1:00:09Z".parse::<DateTime<Local>>().unwrap();
            // 3.5 calculate how many weeks are between two dates
            if tmp > "2023-07-29T1:00:09Z".parse::<DateTime<Local>>().unwrap() {
                (tmp - previous).num_days()/7 + 9327
            } else {
                (tmp - previous).num_days()/7 + 9328
            }
            // dbg!(self.dt);
        };
        // dbg!(issue);
        EconDate { 
            dt: tmp,
            year: tmp.year(),
            issue: issue,
            file_name: format!("Issue_{}_{1}_The_Economist_Full_edition.zip", issue, tmp.format("%Y%m%d")),
        }
    }
    // 3.2 get a date's year

}

impl fmt::Display for EconDate {
    
    // 5. implement a method for the struct that produces the download url
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 3.3 ymd string format
        write!(f, "https://audiocdn.economist.com/sites/default/files/AudioArchive/{0}/{1}/{2}", self.year, self.dt.format("%Y%m%d"), self.file_name)
    }
}

/** `get_Econ_Date` find nearest Saturday from a DateTime<Local>, and return the nearest Saturday in DateTime<Local>.
* It uses the [`chrono`] library to do datetime arithmatics.
# Example
```
use chrono::prelude::*;
use econ::econ_date::get_econ_date;
let test = "2023-01-24T12:00:09Z".parse::<DateTime<Local>>().unwrap();
let nearest_sat = get_econ_date(test);
assert_eq!(nearest_sat, "2023-01-21T12:00:09Z".parse::<DateTime<Local>>().unwrap());
```
*/
pub fn get_econ_date(dt: DateTime<Local>)-> DateTime<Local> {
    for i in 0..7 {
        // 2. add and minus a day
        let next = dt + Duration::days(i);
        let prev = dt - Duration::days(i);
        // 3. Check if the dates' weekday is Saturday
        let econ_date = if next.weekday()== Weekday::Sat {
            next
        } else if prev.weekday()== Weekday::Sat {
            prev
        } else {
            continue;
        };
        return econ_date
    }
    return dt
}
