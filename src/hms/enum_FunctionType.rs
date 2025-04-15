#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
#[repr(transparent)]
pub struct FunctionType(i32);

impl FunctionType {
    pub const JAVA: Self = Self(1);

    pub fn inner(&self) -> i32 {
        self.0
    }

    pub fn to_string(&self) -> ::std::string::String {
        match self {
            Self(1) => ::std::string::String::from("JAVA"),
            Self(val) => val.to_string(),
        }
    }
}

impl ::std::convert::From<i32> for FunctionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ::std::convert::From<FunctionType> for i32 {
    fn from(value: FunctionType) -> i32 {
        value.0
    }
}

impl ::pilota::thrift::Message for FunctionType {
    fn encode<T: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut T,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        __protocol.write_i32(self.inner())?;
        ::std::result::Result::Ok(())
    }

    fn decode<T: ::pilota::thrift::TInputProtocol>(
        __protocol: &mut T,
    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::{thrift::TLengthProtocolExt, Buf};
        let value = __protocol.read_i32()?;
        ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
            ::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                format!("invalid enum value for FunctionType, value: {}", value),
            )
        })?)
    }

    fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
        __protocol: &'a mut T,
    ) -> ::std::pin::Pin<
        ::std::boxed::Box<
            dyn ::std::future::Future<
                    Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                > + Send
                + 'a,
        >,
    > {
        ::std::boxed::Box::pin(async move {
            let value = __protocol.read_i32().await?;
            ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                ::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    format!("invalid enum value for FunctionType, value: {}", value),
                )
            })?)
        })
    }

    fn size<T: ::pilota::thrift::TLengthProtocol>(&self, __protocol: &mut T) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.i32_len(self.inner())
    }
}
