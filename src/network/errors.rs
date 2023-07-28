use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ReceivedError {
    Locking(String),
    Reading(String),
    Sending(String),
}

impl fmt::Display for ReceivedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReceivedError::Locking(msg) => {
                write!(f, "error happened on locking device, msg={}", msg)
            }
            ReceivedError::Reading(msg) => {
                write!(f, "error happened on reading buffers, msg={}", msg)
            }
            ReceivedError::Sending(msg) => {
                write!(f, "error happened on sending data to channel, msg={}", msg)
            }
        }
    }
}

impl ReceivedError {
    pub fn from_locking(msg: String) -> ReceivedError {
        ReceivedError::Locking(msg)
    }

    pub fn from_reading(msg: String) -> ReceivedError {
        ReceivedError::Reading(msg)
    }

    pub fn from_sending(msg: String) -> ReceivedError {
        ReceivedError::Sending(msg)
    }
}

#[derive(PartialEq, Debug)]
pub enum SendingError {
    Locking(String),
    Writing(String),
    Flushing(String),
}

impl fmt::Display for SendingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SendingError::Locking(msg) => {
                write!(f, "error happened on locking device, msg={}", msg)
            }

            SendingError::Writing(msg) => {
                write!(f, "error happened on writing to device, msg={}", msg)
            }

            SendingError::Flushing(msg) => {
                write!(f, "error happened on flushing device, msg={}", msg)
            }
        }
    }
}

impl SendingError {
    pub fn from_locking(msg: String) -> SendingError {
        SendingError::Locking(msg)
    }

    pub fn from_writing(msg: String) -> SendingError {
        SendingError::Writing(msg)
    }

    pub fn from_flushing(msg: String) -> SendingError {
        SendingError::Flushing(msg)
    }
}
