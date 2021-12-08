use super::{range_limit::LimitError, RangeLimitString};
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CryptoString<const L: usize, const H: usize> {
    Raw(RangeLimitString<L, H>),
    Cryptoed([u8; 32]),
}

impl<'s, const L: usize, const H: usize> Into<&'s str> for &'s CryptoString<L, H> {
    fn into(self) -> &'s str {
        match self {
            CryptoString::Raw(raw) => raw.as_ref().as_str(),
            CryptoString::Cryptoed(s) => unsafe { std::str::from_utf8_unchecked(s) },
        }
    }
}

impl<const L: usize, const H: usize> Into<String> for CryptoString<L, H> {
    fn into(self) -> String {
        match self {
            CryptoString::Raw(raw) => raw.into(),
            CryptoString::Cryptoed(arr) => unsafe {
                String::from_utf8_unchecked(arr.into_iter().collect())
            },
        }
    }
}

impl<const L: usize, const H: usize> AsRef<str> for CryptoString<L, H> {
    fn as_ref(&self) -> &str {
        self.into()
    }
}

impl<const L: usize, const H: usize> TryFrom<String> for CryptoString<L, H> {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::Cryptoed(
            value
                .into_bytes()
                .try_into()
                .or_else(|_e| Err("Size Not Match".to_string()))?,
        ))
    }
}

impl<const L: usize, const H: usize> CryptoString<L, H> {
    pub fn new_raw<T: ToString>(data: T) -> Result<CryptoString<L, H>, LimitError> {
        let data = data.to_string();
        Ok(Self::Raw(RangeLimitString::try_from(data)?))
    }
    fn crypto(raw: &str, result: &mut [u8]) {
        let mut hasher = Sha3::keccak256();
        hasher.input_str(&raw);
        hasher.result(result);
        hasher.reset();
    }
    pub fn into_crypto(self) -> Self {
        let res = match self {
            CryptoString::Raw(raw) => {
                let mut res: [u8; 32] = Default::default();
                Self::crypto(&raw, &mut res);
                res
            }
            c => return c,
        };

        Self::Cryptoed(res)
    }
}

impl<const L: usize, const H: usize> Serialize for CryptoString<L, H> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            CryptoString::Raw(r) => {
                let mut st: [u8; 32] = Default::default();
                Self::crypto(&r, &mut st);
                unsafe { String::from_utf8_unchecked(st.clone().into_iter().collect()) }
                    .serialize(serializer)
            }
            CryptoString::Cryptoed(st) => {
                unsafe { String::from_utf8_unchecked(st.clone().into_iter().collect()) }
                    .serialize(serializer)
            }
        }
    }
}

impl<'de, const L: usize, const H: usize> Deserialize<'de> for CryptoString<L, H> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match String::deserialize(deserializer) {
            Ok(res) => {
                if res.len() == 32 {
                    let res: [u8; 32] = res.into_bytes().try_into().unwrap();
                    Ok(Self::Cryptoed(res))
                } else {
                    Ok(Self::Raw(
                        RangeLimitString::<L, H>::try_from(res)
                            .or_else(|e| Err(serde::de::Error::custom(e)))?,
                    ))
                }
            }
            Err(err) => Err(err),
        }
    }
}
