#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wrapper {
    #[prost(string, tag="1")]
    pub api: std::string::String,
    #[prost(oneof="wrapper::Msg", tags="2, 3")]
    pub msg: ::std::option::Option<wrapper::Msg>,
}
pub mod wrapper {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag="2")]
        LoginReq(super::LoginRequest),
        #[prost(message, tag="3")]
        LoginResp(super::LoginResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag="1")]
    pub api: std::string::String,
    #[prost(string, tag="2")]
    pub username: std::string::String,
    #[prost(string, tag="3")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResponse {
    #[prost(string, tag="1")]
    pub api: std::string::String,
    #[prost(bool, tag="2")]
    pub status: bool,
    #[prost(string, tag="3")]
    pub username: std::string::String,
}
