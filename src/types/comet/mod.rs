use enum_repr_derive::{FromEnumToRepr, TryFromReprToEnum};

use comet_gate::CometGate;
use comet_login::CometLogin;
use comet_scene::CometScene;

mod comet_gate;
mod comet_login;
mod comet_scene;

#[repr(i8)]
#[derive(Debug, FromEnumToRepr, TryFromReprToEnum, Copy, Clone)]
pub enum MainCmd {
    Time = 1,
    Login = 2,
    Select = 3,
    Game = 5,
}

#[derive(Debug)]
pub enum ParaCmd {
    CometGate(CometGate),
    CometLogin(CometLogin),
    CometScene(CometScene),
}

impl ParaCmd {
    pub fn get_value(&self) -> u8 {
        match self.clone() {
            ParaCmd::CometGate(num) => u8::from(num.clone()),
            ParaCmd::CometLogin(num) => u8::from(num.clone()),
            ParaCmd::CometScene(num) => u8::from(num.clone()),
        }
    }

    pub fn from_value(main_cmd: &MainCmd, value: u8) -> Result<Self, u8> {
        Ok(match main_cmd {
            MainCmd::Time => Self::CometGate(CometGate::try_from(value)?),
            MainCmd::Login => Self::CometLogin(CometLogin::try_from(value)?),
            MainCmd::Select => Self::CometGate(CometGate::try_from(value)?),
            MainCmd::Game => Self::CometScene(CometScene::try_from(value)?),
        })
    }
}
