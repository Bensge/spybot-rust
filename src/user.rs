#[derive(Debug, Default)]
pub(crate) struct User {
    pub(crate) clid: u64,
    pub(crate) unique_id: String,
    pub(crate) nickname: String,
    pub(crate) is_query_user: bool,
}
