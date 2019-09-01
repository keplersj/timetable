use diesel::pg::data_types::PgTimestamp;

#[derive(Queryable)]
pub struct ScheduledRequest {
    pub id: i32,
    pub hook: String,
    pub time: PgTimestamp,
    pub executed: bool,
}