use serde::{Deserialize, Serialize};

pub trait ZRpcDtAuto {
    fn to_zdt(&self) -> ZRpcDt;
}

pub trait ZRpcDtSerialize: Serialize {}

impl<T: ZRpcDtSerialize> ZRpcDtAuto for T {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::serialize(self)
    }
}

impl ZRpcDtAuto for &str {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::String(self.to_string())
    }
}

impl ZRpcDtAuto for String {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::String(self.clone())
    }
}

impl ZRpcDtAuto for i8 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Int8(*self)
    }
}

impl ZRpcDtAuto for i16 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Int16(*self)
    }
}

impl ZRpcDtAuto for i32 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Int32(*self)
    }
}

impl ZRpcDtAuto for i64 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Int64(*self)
    }
}

impl ZRpcDtAuto for u8 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::UInt8(*self)
    }
}

impl ZRpcDtAuto for u16 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::UInt16(*self)
    }
}

impl ZRpcDtAuto for u32 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::UInt32(*self)
    }
}

impl ZRpcDtAuto for u64 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::UInt64(*self)
    }
}

impl ZRpcDtAuto for f32 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Float32(*self)
    }
}

impl ZRpcDtAuto for f64 {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Float64(*self)
    }
}

impl ZRpcDtAuto for bool {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Bool(*self)
    }
}

impl ZRpcDtAuto for Vec<u8> {
    fn to_zdt(&self) -> ZRpcDt {
        ZRpcDt::Serialized(self.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ZRpcDt {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bool(bool),
    Serialized(Vec<u8>),
    Ok,
}

impl ZRpcDt {
    pub fn serialize<T: Serialize>(t: T) -> Self {
        Self::Serialized(bincode::serialize(&t).expect("Failed to serialize type"))
    }

    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> Result<T, ()> {
        if let Self::Serialized(bytes) = self {
            bincode::deserialize::<T>(bytes).map_err(|_| ())
        } else {
            Err(())
        }
    }
}
