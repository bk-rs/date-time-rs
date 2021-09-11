use chrono::NaiveDate;

mod iter;
pub use iter::DateRangeIterator;

#[derive(Debug, Clone)]
pub struct DateRange {
    pub since: NaiveDate,
    pub until: NaiveDate,
}
