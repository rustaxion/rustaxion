use prost::Message;
use tokio_util::codec::{Framed, FramedParts};

use crate::{
    enums::comet::{comet_login::CometLogin, MainCmd, ParaCmd},
    proto::comet_login::{LanguageType, PlatformType, ReqGameVersion, RetGameVersion},
    server,
    types::{packet::Packet, response::Response, session::SessionData},
};

#[tokio::test]
async fn game_version() {
    let mut session = SessionData {};

    let req = ReqGameVersion {
        r#type: PlatformType::Android as i32,
        language: LanguageType::DefaultLanguage as i32,
    };

    let req = Response {
        main_cmd: MainCmd::Login,
        para_cmd: ParaCmd::CometLogin(CometLogin::RequestGameVersion),
        body: req.encode_to_vec(),
    };

    let res = server::handle(&mut session, Into::<Packet>::into(req)).unwrap();
    assert_eq!(res.len(), 1);

    let body = RetGameVersion::decode(res[0].body.as_slice()).unwrap();
    assert_eq!(body.version, "0.1.0")
}
