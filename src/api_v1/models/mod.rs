mod beatmaps;
mod recent;

pub use beatmaps::GetBeatmapsProps;
pub use recent::{GetUserRecentProp, GetUserRecentResp};

use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub enum Mods {
  None = 0,
  NoFail = 1,
  Easy = 2,
  TouchDevice = 4,
  Hidden = 8,
  HardRock = 16,
  SuddenDeath = 32,
  DoubleTime = 64,
  Relax = 128,
  HalfTime = 256,
  Nightcore = 512, // Only set along with DoubleTime. i.e: NC only gives 576
  Flashlight = 1024,
  Autoplay = 2048,
  SpunOut = 4096,
  Relax2 = 8192,   // Autopilot
  Perfect = 16384, // Only set along with SuddenDeath. i.e: PF only gives 16416
  Key4 = 32768,
  Key5 = 65536,
  Key6 = 131072,
  Key7 = 262144,
  Key8 = 524288,
  FadeIn = 1048576,
  Random = 2097152,
  Cinema = 4194304,
  Target = 8388608,
  Key9 = 16777216,
  KeyCoop = 33554432,
  Key1 = 67108864,
  Key3 = 134217728,
  Key2 = 268435456,
  ScoreV2 = 536870912,
  Mirror = 1073741824,
}
