macro_rules! make_server_error {
    ($($variant:ident),*) => {
        use crate::protos::protobuf::pulsar_api::ServerError as Protobuf_ServerError;

        #[derive(PartialEq, Eq, Debug, Clone)]
        pub enum ServerError {
            $(
                $variant,
            )*
        }

        impl From<Protobuf_ServerError> for ServerError {
            fn from(se: Protobuf_ServerError) -> Self {
                match se {
                    $(
                        Protobuf_ServerError::$variant => Self::$variant,
                    )*
                }
            }
        }
    };
}

make_server_error!(
    UnknownError,
    MetadataError,
    PersistenceError,
    AuthenticationError,
    AuthorizationError,
    ConsumerBusy,
    ServiceNotReady,
    ProducerBlockedQuotaExceededError,
    ProducerBlockedQuotaExceededException,
    ChecksumError,
    UnsupportedVersionError,
    TopicNotFound,
    SubscriptionNotFound,
    ConsumerNotFound,
    TooManyRequests,
    TopicTerminatedError,
    ProducerBusy,
    InvalidTopicName,
    IncompatibleSchema,
    ConsumerAssignError,
    TransactionCoordinatorNotFound,
    InvalidTxnStatus,
    NotAllowedError,
    TransactionConflict,
    TransactionNotFound,
    ProducerFenced
);
