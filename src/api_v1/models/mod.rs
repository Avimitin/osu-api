mod beatmaps;
mod recent;

pub use beatmaps::GetBeatmapsProps;
pub use recent::{GetUserRecentProp, GetUserRecentResp};

use chrono::{TimeZone, Utc};
use paste::paste;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OsuMode {
  Standard,
  Taiko,
  CatchTheBeat,
  Mania,
}

impl ToString for OsuMode {
  fn to_string(&self) -> String {
    self.as_ref().to_string()
  }
}

impl AsRef<str> for OsuMode {
  fn as_ref(&self) -> &str {
    use OsuMode::*;

    match self {
      Standard => "0",
      Taiko => "1",
      CatchTheBeat => "2",
      Mania => "3",
    }
  }
}

#[derive(Debug)]
pub enum UserId<'u> {
  Id(u64),
  Username(&'u str),
}

impl From<u64> for UserId<'_> {
  fn from(id: u64) -> Self {
    Self::Id(id)
  }
}

impl<'a> From<&'a str> for UserId<'a> {
  fn from(name: &'a str) -> Self {
    Self::Username(name)
  }
}

bitflags::bitflags! {
  pub struct ModsFlag: u64 {
    const NONE              = 0;
    const NOFAIL            = 1;
    const EASY              = 2;
    const TOUCHDEVICE       = 4;
    const HIDDEN            = 8;
    const HARDROCK          = 16;
    const SUDDENDEATH       = 32;
    const DOUBLETIME        = 64;
    const RELAX             = 128;
    const HALFTIME          = 256;
    const NIGHTCORE         = 512 | Self::DOUBLETIME.bits;
    const FLASHLIGHT        = 1024;
    const AUTOPLAY          = 2048;
    const SPUNOUT           = 4096;
    const RELAX2            = 8192;    // Autopilot
    const PERFECT           = 16384 | Self::SUDDENDEATH.bits;
    const KEY4              = 32768;
    const KEY5              = 65536;
    const KEY6              = 131072;
    const KEY7              = 262144;
    const KEY8              = 524288;
    const FADEIN            = 1048576;
    const RANDOM            = 2097152;
    const CINEMA            = 4194304;
    const TARGET            = 8388608;
    const KEY9              = 16777216;
    const KEYCOOP           = 33554432;
    const KEY1              = 67108864;
    const KEY3              = 134217728;
    const KEY2              = 268435456;
    const SCOREV2           = 536870912;
    const MIRROR            = 1073741824;
    const KEYMOD            = Self::KEY1.bits
                            | Self::KEY2.bits
                            | Self::KEY3.bits
                            | Self::KEY4.bits
                            | Self::KEY5.bits
                            | Self::KEY6.bits
                            | Self::KEY7.bits
                            | Self::KEY8.bits
                            | Self::KEY9.bits
                            | Self::KEYCOOP.bits;
    const FREEMODALLOWED    = Self::NOFAIL.bits
                            | Self::EASY.bits
                            | Self::HIDDEN.bits
                            | Self::HARDROCK.bits
                            | Self::SUDDENDEATH.bits
                            | Self::FLASHLIGHT.bits
                            | Self::FADEIN.bits
                            | Self::RELAX.bits
                            | Self::RELAX2.bits
                            | Self::SPUNOUT.bits
                            | Self::KEYMOD.bits;
    const SCOREINCREASEMODS = Self::HIDDEN.bits
                            | Self::HARDROCK.bits
                            | Self::DOUBLETIME.bits
                            | Self::FLASHLIGHT.bits
                            | Self::FADEIN.bits;
  }
}

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

fn s_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
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

fn s_to_datetime<'de, D>(d: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
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
        fn [<s_to_ $t>]<'de, D>(d: D) -> Result<$t, D::Error>
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

s_to_scalar![u32, u64];
