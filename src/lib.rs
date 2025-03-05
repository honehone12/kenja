pub mod search_engines;
pub mod services {
    pub mod anime_search;

    pub(crate) const INTERNAL_ERROR: &'static str = "internal server error";
    pub(crate) const INVALID_ARGUMENT: &'static str = "invalid argument";
}
pub mod documents {
    pub mod anime_search;
}
