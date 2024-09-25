
use std::{str::FromStr, str, fmt};
use std::fmt::Formatter;
use std::hash::{DefaultHasher, Hash, Hasher};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
pub use error_code_derive::ToErrorInfo;
pub struct ErrorInfo<T> {
    pub app_code: T,
    pub code: &'static str,
    pub hash:String,
    pub client_msg: &'static str,
    pub server_msg: String,
}
pub trait ToErrorInfo {
    type T:FromStr;
    fn to_error_info(&self) ->ErrorInfo<Self::T>;
}
impl<T> ErrorInfo<T>
where T:FromStr ,
    <T as FromStr>::Err:fmt::Debug
{
    pub fn new(app_code: &str, code: &'static str, client_msg: &'static str, server_msg:  impl fmt::Display) -> Self {
        let server_msg = server_msg.to_string();
        let mut hasher = DefaultHasher::new();
        server_msg.hash(&mut hasher);
       let hash = hasher.finish();
        let hash = URL_SAFE_NO_PAD.encode(&hash.to_be_bytes());
       Self {
           app_code:T::from_str(app_code).expect("Can not parse app_code"),
           code,
           hash,
           client_msg,
           server_msg:server_msg.to_string(),
       }
    }
}
impl <T> ErrorInfo<T>{
    pub fn client_msg(&self)->&str{
        if self.client_msg.is_empty(){
            &self.server_msg
        }else {
            self.client_msg
        }

    }
}
// Display for client facing error message
impl<T> fmt::Display for ErrorInfo<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"[{}-{}] {}",self.code,self.hash,self.client_msg())
    }
}
impl<T> fmt::Debug for ErrorInfo<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"[{}-{}] {}",self.code,self.hash,self.client_msg())
    }
}