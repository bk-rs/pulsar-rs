use crate::commands::FlowCommand;

use super::HalfRequest;

pub struct ConsumerFlowHalfRequest {}
impl HalfRequest for ConsumerFlowHalfRequest {
    type Request = FlowCommand;
    type Error = ConsumerFlowHalfRequestError;
}

make_x_half_request_error!(
    ConsumerFlow;
);
