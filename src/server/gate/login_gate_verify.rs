use std::time::SystemTime;

use anyhow::Context;
use prost::Message;

use crate::{
    enums::comet::{comet_gate::CometGate, MainCmd, ParaCmd},
    proto::comet_gate::{LoginGateVerify, NtfGameTime},
    types::{response::Response, session::SessionData},
};

#[rustfmt::skip]
pub fn handle(session: &mut SessionData, buffer: Vec<u8>) -> anyhow::Result<Vec<Response>> {
    let req = LoginGateVerify::decode(buffer.as_slice()).context("Failed to decode LoginGateVerify.")?;
    let mut responses = Vec::<Response>::with_capacity(2);

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let time = NtfGameTime {
        gametime: u32::try_from(now.as_secs())?,
    };

    responses.push(Response {
        main_cmd: MainCmd::Time,
        para_cmd: ParaCmd::CometGate(CometGate::NtfGameTime),
        body: time.encode_to_vec()
    });

    /*
    var data = Serializer.Deserialize<cometGate.LoginGateVerify>(new MemoryStream(msgContent));
    var accId = data.accId;

    var account = Server.Database.GetAccount(accId);
    if (account == null)
        return;

    account.sessionId = (uint)sessionId;
    Server.Database.UpdateAccount(account);

    var userInfoList = new cometGate.SelectUserInfoList();

    if (account.charId != 0)
    {
        userInfoList.userList.Add(new cometGate.SelectUserInfo { charId = (uint)account.charId, accStates = 0, });
    }

    ServerLogger.LogInfo($"LoginGateVerify: [{((userInfoList.userList.Count > 0) ? ("{ charId: " + account.charId + ", accStates: 0 }") : "")}]");
    Index.Instance.GatePackageQueue.Enqueue(
        new Index.GamePackage()
        {
            MainCmd = (uint)cometGate.MainCmd.MainCmd_Select,
            ParaCmd = (uint)cometGate.ParaCmd.ParaCmd_SelectUserInfoList,
            Data = Index.ObjectToByteArray(userInfoList),
        }
    );
    */

    Ok(responses)
}
