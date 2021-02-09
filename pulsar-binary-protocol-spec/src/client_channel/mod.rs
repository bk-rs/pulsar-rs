#[cfg(feature = "with-asynchronous")]
pub use async_channel::{Receiver as AC_Receiver, Sender as AC_Sender};
#[cfg(feature = "with-asynchronous")]
pub use futures_channel::oneshot::{Receiver as FC_Receiver, Sender as FC_Sender};
#[cfg(not(feature = "with-asynchronous"))]
pub use std::sync::mpsc::{
    Receiver as FC_Receiver, Receiver as AC_Receiver, Sender as AC_Sender, Sender as FC_Sender,
};

pub mod handler_channel_storage;

pub use handler_channel_storage::{HandlerChannelStorage, HandlerChannelStorageItem};
