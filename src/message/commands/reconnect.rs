use crate::message::commands::ServerMessageParseError;
use crate::message::commands::ServerMessageParseError::MismatchedCommand;
use crate::message::IRCMessage;
use derivative::Derivative;
use std::convert::TryFrom;

#[readonly::make]
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq)]
pub struct ReconnectMessage {
    #[derivative(PartialEq = "ignore")]
    source: IRCMessage,
}

impl TryFrom<IRCMessage> for ReconnectMessage {
    type Error = ServerMessageParseError;

    fn try_from(source: IRCMessage) -> Result<ReconnectMessage, ServerMessageParseError> {
        if source.command == "RECONNECT" {
            Ok(ReconnectMessage { source })
        } else {
            Err(MismatchedCommand())
        }
    }
}

impl From<ReconnectMessage> for IRCMessage {
    fn from(msg: ReconnectMessage) -> IRCMessage {
        msg.source
    }
}
