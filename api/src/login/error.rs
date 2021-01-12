#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid command {0}[{0:x}]")]
    InvalidCommand(u16),
    #[error("Invalid packet size {0}")]
    InvalidPacketSize(usize),
    #[error("Packet incomplete - need {0} bytes")]
    PacketIncomplete(usize),
}
