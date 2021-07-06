use crate::data_order::DataOrder;
use std::any::type_name;
use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ConnectorAgentError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorAgentError {
    /// The required type does not same as the schema defined.
    #[error("Data type unexpected: {0:?} expected, {1} found.")]
    TypeCheckFailed(String, &'static str),

    #[error("Index operation out of bound.")]
    OutOfBound,

    #[error("Data order not supported {0:?}.")]
    UnsupportedDataOrder(DataOrder),

    #[error("Cannot resolve data order: got {0:?} from source, {1:?} from destination.")]
    CannotResolveDataOrder(Vec<DataOrder>, Vec<DataOrder>),

    #[error("Cannot produce a {0}, context: {1}.")]
    CannotProduce(&'static str, ProduceContext),

    #[error("Allocate is already called.")]
    DuplicatedAllocation,

    #[error("Destination has not been allocated yet.")]
    DestinationNotAllocated,

    #[error("No conversion rule from {0} to {1}.")]
    NoConversionRule(String, String),

    #[error("Only support single query with SELECT statement, got {0}.")]
    SQLQueryNotSupported(String),

    #[error("Only support partition on SPJ query, got {0}.")]
    SQLQueryPartitionNotSupported(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    PostgresPoolError(#[from] r2d2::Error),

    #[error(transparent)]
    PostgresError(#[from] postgres::Error),

    #[error(transparent)]
    MysqlError(#[from] r2d2_mysql::mysql::Error),

    #[error(transparent)]
    MysqlUrlError(#[from] r2d2_mysql::mysql::UrlError),

    #[error(transparent)]
    SQLiteError(#[from] rusqlite::Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    #[error(transparent)]
    SQLParserError(#[from] sqlparser::parser::ParserError),

    #[error(transparent)]
    RegexError(#[from] regex::Error),

    #[error(transparent)]
    NdArrayShapeError(#[from] ndarray::ShapeError),

    #[error(transparent)]
    ArrowError(#[from] arrow::error::ArrowError),

    #[error(transparent)]
    HexError(#[from] hex::FromHexError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ConnectorAgentError {
    pub fn cannot_produce<T>(context: Option<String>) -> Self {
        ConnectorAgentError::CannotProduce(type_name::<T>(), context.into())
    }
}

#[derive(Debug)]
pub enum ProduceContext {
    NoContext,
    Context(String),
}

impl From<Option<String>> for ProduceContext {
    fn from(val: Option<String>) -> Self {
        match val {
            Some(c) => ProduceContext::Context(c),
            None => ProduceContext::NoContext,
        }
    }
}

impl fmt::Display for ProduceContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProduceContext::NoContext => write!(f, "No Context"),
            ProduceContext::Context(s) => write!(f, "{}", s),
        }
    }
}
