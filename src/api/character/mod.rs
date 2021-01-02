use async_std::io;
use async_std::io::{Read, ReadExt};
use tracing::error;

const ACK_CONNECT: u32 = 0x2711;
const ACK_ACC_REQ: u32 = 0x2713;
const REQ_ACC_DATA: u32 = 0x2717;
const KEEP_ALIVE: u32 = 0x2718;
const ACC_INFO_ACK: u32 = 0x2721;
const ACK_CHANGE_SEX: u32 = 0x2723;
const ACK_GLOBAL_ACC_REG: u32 = 0x2726;
const ACC_BAN_NOTIFICATION: u32 = 0x2731;
const ASK_KICK: u32 = 0x2734;
const UPD_IP: u32 = 0x2735;
const VIP_ACK: u32 = 0x2743;

#[derive(Debug)]
pub enum Request {
    AcknowledgeConnection,
    AcknowledgeAccountRequest,
    RequestAccountData,
    KeepAlive,
    AccountInfoAcknowledge,
    AcknowledgeChangeSex,
    AcknowledgeGlobalAccountRegistration,
}

impl Request {
    pub async fn parse(mut reader: impl Read + Unpin) -> Result<Self, io::Error> {
        let mut command_buf = [0u8; 4];
        reader.read_exact(&mut command_buf).await?;
        match u32::from_le_bytes(command_buf) {
            // ACK_CONNECT => chlogif_parse_ackconnect(fd),
            // ACK_ACC_REQ => chlogif_parse_ackaccreq(fd),
            // REQ_ACC_DATA => chlogif_parse_reqaccdata(fd),
            // KEEP_ALIVE => chlogif_parse_keepalive(fd),
            // ACC_INFO_ACK => chlogif_parse_AccInfoAck(fd),
            // ACK_CHANGE_SEX => chlogif_parse_ackchangesex(fd),
            // ACK_GLOBAL_ACC_REG => chlogif_parse_ack_global_accreg(fd),
            // ACC_BAN_NOTIFICATION => chlogif_parse_accbannotification(fd),
            // ASK_KICK => chlogif_parse_askkick(fd),
            // UPD_IP => chlogif_parse_updip(fd),
            // VIP_ACK => chlogif_parse_vipack(fd),
            unknown => {
                error!("Unknown command {}", unknown);
                todo!("implement me");
            }
        }
    }
}
