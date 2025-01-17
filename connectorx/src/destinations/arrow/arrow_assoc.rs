use crate::errors::{ConnectorAgentError, Result};
use arrow::array::{
    ArrayBuilder, BooleanBuilder, Float64Builder, Int32Builder, Int64Builder, StringBuilder,
};
use arrow::datatypes::DataType as ArrowDataType;
use arrow::datatypes::Field;
use chrono::{Date, DateTime, Utc};
use fehler::throws;

/// Associate arrow builder with native type
pub trait ArrowAssoc {
    type Builder: ArrayBuilder + Send;

    fn builder(nrows: usize) -> Self::Builder;
    fn append(builder: &mut Self::Builder, value: Self) -> Result<()>;
    fn field(header: &str) -> Field;
}

impl ArrowAssoc for i32 {
    type Builder = Int32Builder;

    fn builder(nrows: usize) -> Int32Builder {
        Int32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int32Builder, value: i32) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, false)
    }
}

impl ArrowAssoc for Option<i32> {
    type Builder = Int32Builder;

    fn builder(nrows: usize) -> Int32Builder {
        Int32Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int32Builder, value: Option<i32>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::UInt64, true)
    }
}

impl ArrowAssoc for i64 {
    type Builder = Int64Builder;

    fn builder(nrows: usize) -> Int64Builder {
        Int64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int64Builder, value: i64) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Int64, false)
    }
}

impl ArrowAssoc for Option<i64> {
    type Builder = Int64Builder;

    fn builder(nrows: usize) -> Int64Builder {
        Int64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Int64Builder, value: Option<i64>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Int64, false)
    }
}

impl ArrowAssoc for f64 {
    type Builder = Float64Builder;

    fn builder(nrows: usize) -> Float64Builder {
        Float64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: f64) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, false)
    }
}

impl ArrowAssoc for Option<f64> {
    type Builder = Float64Builder;

    fn builder(nrows: usize) -> Float64Builder {
        Float64Builder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Option<f64>) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Float64, true)
    }
}

impl ArrowAssoc for bool {
    type Builder = BooleanBuilder;

    fn builder(nrows: usize) -> BooleanBuilder {
        BooleanBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: bool) {
        builder.append_value(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Boolean, false)
    }
}

impl ArrowAssoc for Option<bool> {
    type Builder = BooleanBuilder;

    fn builder(nrows: usize) -> BooleanBuilder {
        BooleanBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        builder.append_option(value)?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Boolean, true)
    }
}

impl ArrowAssoc for String {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> StringBuilder {
        StringBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: String) {
        builder.append_value(value.as_str())?;
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}

impl ArrowAssoc for Option<String> {
    type Builder = StringBuilder;

    fn builder(nrows: usize) -> StringBuilder {
        StringBuilder::new(nrows)
    }

    #[throws(ConnectorAgentError)]
    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s.as_str())?,
            None => builder.append_null()?,
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, true)
    }
}

impl ArrowAssoc for DateTime<Utc> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: DateTime<Utc>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Option<DateTime<Utc>> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Option<DateTime<Utc>>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Date<Utc> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Date<Utc>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Option<Date<Utc>> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Option<Date<Utc>>) -> Result<()> {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}
