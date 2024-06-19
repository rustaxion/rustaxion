use enum_repr_derive::{FromEnumToRepr, TryFromReprToEnum};

#[repr(u8)]
#[derive(Debug, FromEnumToRepr, TryFromReprToEnum, Copy, Clone)]
pub enum CometLogin {
    RequestRegAccount = 1,
    ResponseRegAccount = 2,
    RequestLoginAccount = 3,
    ResponseLoginAccount = 4,
    RequestFindPassword = 5,
    ResponseFindPassword = 6,
    LoginError = 10,
    RequestQuickToken = 11,
    ResponseQuickToken = 12,
    RequestQuickLogin = 13,
    ResponseQuickLogin = 14,
    RequestThirdLogin = 15,
    ResponseThirdLogin = 16,
    RequestBindAccount = 17,
    ResponseBindAccount = 18,
    RequestAnnouncement = 19,
    ResponseAnnouncement = 20,
    RequestGameVersion = 21,
    ResponseGameVersion = 22,
    RequestBiliLogin = 23,
    ResponseBiliLogin = 24,
}
