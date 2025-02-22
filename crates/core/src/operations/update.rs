// TODO: complete update logic in the network

pub(crate) use self::messages::UpdateMsg;
use crate::{client_events::ClientId, node::ConnectionBridge};

use super::{op_trait::Operation, OpError};

pub(crate) struct UpdateOp {}

pub(crate) struct UpdateResult {}

impl TryFrom<UpdateOp> for UpdateResult {
    type Error = OpError;

    fn try_from(_value: UpdateOp) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Operation for UpdateOp {
    type Message = UpdateMsg;
    type Result = UpdateResult;

    fn load_or_init(
        _op_storage: &crate::node::OpManager,
        _msg: &Self::Message,
    ) -> Result<super::OpInitialization<Self>, OpError> {
        todo!()
    }

    fn id(&self) -> &crate::message::Transaction {
        todo!()
    }

    fn process_message<'a, CB: ConnectionBridge>(
        self,
        _conn_manager: &'a mut CB,
        _op_storage: &'a crate::node::OpManager,
        _input: Self::Message,
        _client_id: Option<ClientId>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<super::OperationResult, OpError>> + Send + 'a>,
    > {
        todo!()
    }
}

mod messages {
    use serde::{Deserialize, Serialize};

    use crate::message::{InnerMessage, Transaction};

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    pub(crate) enum UpdateMsg {}

    impl InnerMessage for UpdateMsg {
        fn id(&self) -> &Transaction {
            todo!()
        }
    }
}
