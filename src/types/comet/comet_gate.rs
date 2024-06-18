#[derive(Debug)]
pub enum CometGate {
    NtfGameTime = 1,
    RequestUserGameTime = 2,
    ResponseUserGameTime = 3,
    LoginGateError = 4,
    LoginGateVerify = 5,
    SelectUserInfoList = 6,
    CreateCharacter = 7,
    EnterGame = 8,
}

impl CometGate {
    pub fn get_value(&self) -> u8 {
        match self {
            CometGate::NtfGameTime => CometGate::NtfGameTime as u8,
            CometGate::RequestUserGameTime => CometGate::RequestUserGameTime as u8,
            CometGate::ResponseUserGameTime => CometGate::ResponseUserGameTime as u8,
            CometGate::LoginGateError => CometGate::LoginGateError as u8,
            CometGate::LoginGateVerify => CometGate::LoginGateVerify as u8,
            CometGate::SelectUserInfoList => CometGate::SelectUserInfoList as u8,
            CometGate::CreateCharacter => CometGate::CreateCharacter as u8,
            CometGate::EnterGame => CometGate::EnterGame as u8,
        }
    }

    pub fn from_value(value: u8) -> Result<Self, std::io::Error> {
        match value {
            1 => Ok(Self::NtfGameTime),
            2 => Ok(Self::RequestUserGameTime),
            3 => Ok(Self::ResponseUserGameTime),
            4 => Ok(Self::LoginGateError),
            5 => Ok(Self::LoginGateVerify),
            6 => Ok(Self::SelectUserInfoList),
            7 => Ok(Self::CreateCharacter),
            8 => Ok(Self::EnterGame),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{} is not a valid comet_gate", value)))
        }
    }
}
