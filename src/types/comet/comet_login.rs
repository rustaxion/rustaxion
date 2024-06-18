#[derive(Debug)]
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

impl CometLogin {
    pub fn get_value(&self) -> u8 {
        match self {
            CometLogin::RequestRegAccount => CometLogin::RequestRegAccount as u8,
            CometLogin::ResponseRegAccount => CometLogin::ResponseRegAccount as u8,
            CometLogin::RequestLoginAccount => CometLogin::RequestLoginAccount as u8,
            CometLogin::ResponseLoginAccount => CometLogin::ResponseLoginAccount as u8,
            CometLogin::RequestFindPassword => CometLogin::RequestFindPassword as u8,
            CometLogin::ResponseFindPassword => CometLogin::ResponseFindPassword as u8,
            CometLogin::LoginError => CometLogin::LoginError as u8,
            CometLogin::RequestQuickToken => CometLogin::RequestQuickToken as u8,
            CometLogin::ResponseQuickToken => CometLogin::ResponseQuickToken as u8,
            CometLogin::RequestQuickLogin => CometLogin::RequestQuickLogin as u8,
            CometLogin::ResponseQuickLogin => CometLogin::ResponseQuickLogin as u8,
            CometLogin::RequestThirdLogin => CometLogin::RequestThirdLogin as u8,
            CometLogin::ResponseThirdLogin => CometLogin::ResponseThirdLogin as u8,
            CometLogin::RequestBindAccount => CometLogin::RequestBindAccount as u8,
            CometLogin::ResponseBindAccount => CometLogin::ResponseBindAccount as u8,
            CometLogin::RequestAnnouncement => CometLogin::RequestAnnouncement as u8,
            CometLogin::ResponseAnnouncement => CometLogin::ResponseAnnouncement as u8,
            CometLogin::RequestGameVersion => CometLogin::RequestGameVersion as u8,
            CometLogin::ResponseGameVersion => CometLogin::ResponseGameVersion as u8,
            CometLogin::RequestBiliLogin => CometLogin::RequestBiliLogin as u8,
            CometLogin::ResponseBiliLogin => CometLogin::ResponseBiliLogin as u8,
        }
    }

    pub fn from_value(value: u8) -> Result<Self, std::io::Error> {
        match value {
            1 => Ok(Self::RequestRegAccount),
            2 => Ok(Self::ResponseRegAccount),
            3 => Ok(Self::RequestLoginAccount),
            4 => Ok(Self::ResponseLoginAccount),
            5 => Ok(Self::RequestFindPassword),
            6 => Ok(Self::ResponseFindPassword),
            10 => Ok(Self::LoginError),
            11 => Ok(Self::RequestQuickToken),
            12 => Ok(Self::ResponseQuickToken),
            13 => Ok(Self::RequestQuickLogin),
            14 => Ok(Self::ResponseQuickLogin),
            15 => Ok(Self::RequestThirdLogin),
            16 => Ok(Self::ResponseThirdLogin),
            17 => Ok(Self::RequestBindAccount),
            18 => Ok(Self::ResponseBindAccount),
            19 => Ok(Self::RequestAnnouncement),
            20 => Ok(Self::ResponseAnnouncement),
            21 => Ok(Self::RequestGameVersion),
            22 => Ok(Self::ResponseGameVersion),
            23 => Ok(Self::RequestBiliLogin),
            24 => Ok(Self::ResponseBiliLogin),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{} is not a valid comet_login", value)))
        }
    }
}
