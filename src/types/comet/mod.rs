mod comet_gate;
mod comet_login;
mod comet_scene;

use comet_gate::CometGate;
use comet_login::CometLogin;
use comet_scene::CometScene;

#[derive(Debug)]
pub enum MainCmd {
    Time = 1,
    Login = 2,
    Select = 3,
    Game = 5
}

#[derive(Debug)]
pub enum ParaCmd {
    CometGate(CometGate),
    CometLogin(CometLogin),
    CometScene(CometScene)
}

impl MainCmd {
    pub fn get_value(&self) -> i8 {
        match self {
            MainCmd::Time => 1i8,
            MainCmd::Login => 2i8,
            MainCmd::Select => 3i8,
            MainCmd::Game => 5i8
        }
    }

    pub fn from_value(value: i8) -> Result<Self, std::io::Error> {
        match value {
            1 => Ok(Self::Time),
            2 => Ok(Self::Login),
            3 => Ok(Self::Select),
            5 => Ok(Self::Game),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{} is not a valid main_cmd", value)))
        }
    }
}

impl ParaCmd {
    pub fn get_value(&self) -> u8 {
        match self {
            ParaCmd::CometGate(num) => num.get_value(),
            ParaCmd::CometLogin(num) => num.get_value(),
            ParaCmd::CometScene(num) => num.get_value(),
        }
    }

    pub fn from_value(main_cmd: &MainCmd, value: u8) -> Result<Self, std::io::Error> {
        Ok(match main_cmd {
            MainCmd::Time => Self::CometGate(CometGate::from_value(value)?),
            MainCmd::Login => Self::CometLogin(CometLogin::from_value(value)?),
            MainCmd::Select => Self::CometGate(CometGate::from_value(value)?),
            MainCmd::Game => Self::CometScene(CometScene::from_value(value)?),
        })
    }
}
