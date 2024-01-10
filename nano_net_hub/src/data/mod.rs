use lazy_static::lazy_static;
use crate::config::database::get_conn;
use crate::data::device_data::Tsdb;

pub mod device_data;
pub mod sqlite_data;

lazy_static! {
    #[cfg(feature = "sqlite_time_series")]
    static ref TSDB: Box<dyn Tsdb+ Send + Sync> = Box::new(sqlite_data::SqlxTsdb::new(get_conn()));
}

pub fn get_tsdb() -> &'static Box<dyn Tsdb + Send + Sync> {
    &*TSDB
}