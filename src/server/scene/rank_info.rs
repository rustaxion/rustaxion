use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{ comet_scene::CometScene, MainCmd, ParaCmd },
    proto::comet_scene::{ RankTopType, ReqRankInfo, RetRankInfo },
    types::{ response::Response, session::SessionData },
};

#[rustfmt::skip]
pub async fn handle(_session: &mut SessionData, _db: sea_orm::DatabaseConnection, body: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = ReqRankInfo::decode(body.as_slice()).context("Failed to decode ReqRankInfo.")?;
    let _rank_top_type = RankTopType::try_from(req.r#type).context("Failed to parse ReqRankInfo.RankTopType.")?;

    // match rank_top_type {
    //     RankTopType::RtpDuanwei => unimplemented!(),
    //     RankTopType::RtpRank4 => unimplemented!(),
    //     RankTopType::RtpRank6 => unimplemented!(),
    //     RankTopType::RtpNormal => unimplemented!(),
    //     RankTopType::RtpTotal => unimplemented!(),
    //     RankTopType::RtpTotal4K => unimplemented!(),
    //     RankTopType::RtpTotal6K => unimplemented!(),
    //     RankTopType::RtpTotal8K => unimplemented!(),
    //     RankTopType::RtpArcade => unimplemented!(),
    //     RankTopType::RtpLevel => unimplemented!(),
    //     RankTopType::RtpBf => unimplemented!(),
    //     RankTopType::RtpActive => unimplemented!(),
    // }

    /* NOTE(arjix): Calculating the top players per type, on demand may be slow and sounds like a pain ngl.
                    So I will leave this blank for now, and will come back to it once I've finalized the database structure.
    */

    let ret = RetRankInfo {
        list: vec![],
        r#type: req.r#type
    };

    Ok(vec![Response {
        main_cmd: MainCmd::Game,
        para_cmd: ParaCmd::CometScene(CometScene::ResponseRankInfo),
        body: ret.encode_to_vec()
    }])
}
