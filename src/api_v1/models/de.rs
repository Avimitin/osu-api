use super::{beatmaps::Approval, ModsFlag};
use chrono::{TimeZone, Utc};
use paste::paste;
use serde::{Deserialize, Deserializer};

pub(crate) fn s_to_mods_flags<'de, D>(d: D) -> Result<ModsFlag, D::Error>
where
  D: Deserializer<'de>,
{
  let mods: String = Deserialize::deserialize(d)?;
  let mods: u64 = mods
    .parse()
    .expect("Expect u64 but get unexpected type when parsing mods");
  let flag =
    ModsFlag::from_bits(mods).ok_or_else(|| serde::de::Error::custom("invalid mods bitflag"))?;
  Ok(flag)
}

pub(crate) fn s_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  let is: String = Deserialize::deserialize(d)?;
  let is = match is.as_str() {
    "0" => false,
    _ => true,
  };
  Ok(is)
}

pub(crate) fn s_to_datetime<'de, D>(d: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let date: String = Deserialize::deserialize(d)?;
  let date = Utc.datetime_from_str(&date, "%F %T").map_err(|err| {
    serde::de::Error::custom(format!(
      "response datetime is not in expecting format: {err}"
    ))
  })?;
  Ok(date)
}

macro_rules! s_to_scalar {
  ($($t:tt),+) => {
    $(
      paste! {
        pub(crate) fn [<s_to_ $t>]<'de, D>(d: D) -> Result<$t, D::Error>
        where
          D: Deserializer<'de>
        {
          let s: String = Deserialize::deserialize(d)?;
          let ret: $t = s.parse()
                         .map_err(|err| serde::de::Error::custom(
                            format!("Expecting {} but found unexpecting data: {err}", stringify!($t))
                          ))?;
          Ok(ret)
        }
      }
    )+
  }
}

s_to_scalar![u16, u32, u64, f32, f64];

pub(crate) fn s_to_approval<'de, D>(d: D) -> Result<Approval, D::Error>
where
  D: Deserializer<'de>,
{
  let approved: String = Deserialize::deserialize(d)?;
  let a = match approved.as_str() {
    "-2" => Approval::Graveyard,
    "-1" => Approval::WIP,
    "0" => Approval::Pending,
    "1" => Approval::Ranked,
    "2" => Approval::Approved,
    "3" => Approval::Qualified,
    "4" => Approval::Loved,
    _ => {
      return Err(serde::de::Error::custom(format!(
        "Unknown approved state {}",
        approved
      )))
    }
  };
  Ok(a)
}
