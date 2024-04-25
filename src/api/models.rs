#[derive(Debug ,serde::Deserialize)]
pub(crate) struct Response<T> {
    pub(crate) data: T,
}

#[derive(Debug ,serde::Deserialize)]
pub(crate) struct LoginData {
    pub(crate) token: String,
}

#[derive(Debug,serde::Deserialize)]
pub(crate) struct ProfileData {
    pub(crate) id: i64,
    pub(crate) email: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) phone: String
}



