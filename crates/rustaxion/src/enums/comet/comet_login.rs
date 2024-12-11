use enum_repr_derive::{FromEnumToRepr, TryFromReprToEnum};

#[repr(u8)]
#[derive(Debug, FromEnumToRepr, TryFromReprToEnum, Copy, Clone)]
pub enum CometLogin {
    RequestRegAccount = 1,
    ReturnRegAccount = 2,
    RequestLoginAccount = 3,
    ReturnLoginAccount = 4,
    RequestFindPassword = 5,
    ReturnFindPassword = 6,
    LoginError = 10,
    RequestQuickToken = 11,
    ReturnQuickToken = 12,
    RequestQuickLogin = 13,
    ReturnQuickLogin = 14,
    RequestThirdLogin = 15,
    ReturnThirdLogin = 16,
    RequestBindAccount = 17,
    ReturnBindAccount = 18,
    RequestAnnouncement = 19,
    ReturnAnnouncement = 20,
    RequestGameVersion = 21,
    ReturnGameVersion = 22,
    RequestBiliLogin = 23,
    ReturnBiliLogin = 24,
}
