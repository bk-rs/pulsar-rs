use std::collections::HashMap;

use crate::{commands::MessageCommand, types::ConsumerId};

pub type PendingMessages = HashMap<ConsumerId, Vec<MessageCommand>>;
