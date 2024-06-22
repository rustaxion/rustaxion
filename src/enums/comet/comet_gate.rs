use enum_repr_derive::{FromEnumToRepr, TryFromReprToEnum};

#[repr(u8)]
#[derive(Debug, FromEnumToRepr, TryFromReprToEnum, Copy, Clone)]
pub enum CometGate {
    NotifyGameTime = 1,
    RequestUserGameTime = 2,
    ResponseUserGameTime = 3,
    LoginGateError = 4,
    LoginGateVerify = 5,
    SelectUserInfoList = 6,
    CreateCharacter = 7,
    EnterGame = 8,
}
