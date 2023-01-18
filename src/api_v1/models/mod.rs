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
  None,
  NoFail,
  Easy,
  TouchDevice,
  Hidden,
  HardRock,
  SuddenDeath,
  DoubleTime,
  Relax,
  HalfTime,
  Nightcore, // Only set along with DoubleTime. i.e: NC only gives 576
  Flashlight,
  Autoplay,
  SpunOut,
  Relax2,  // Autopilot
  Perfect, // Only set along with SuddenDeath. i.e: PF only gives 16416
  Key4,
  Key5,
  Key6,
  Key7,
  Key8,
  FadeIn,
  Random,
  Cinema,
  Target,
  Key9,
  KeyCoop,
  Key1,
  Key3,
  Key2,
  ScoreV2,
  Mirror,
}

impl From<Mods> for u64 {
  fn from(mods: Mods) -> Self {
    // special case
    if let Mods::None = mods {
      return 0;
    }

    let ret = 1 << ((mods as u64) - 1);

    match ret {
      // NC should be set along with DT
      512 => {
        let mods = Mods::DoubleTime;
        let dt: u64 = mods.into();
        ret | dt
      }
      // PF should be set along with SD
      16384 => {
        let mods = Mods::SuddenDeath;
        let sd: u64 = mods.into();
        ret | sd
      }
      _ => ret,
    }
  }
}

#[test]
fn test_mods_bit() {
  let eq = |get: u64, expect: u64| {
    assert_eq!(get, expect);
  };

  let mods = Mods::HardRock;
  eq(mods.into(), 16);

  let mods = Mods::Nightcore;
  eq(mods.into(), 576);

  let mods = Mods::Perfect;
  eq(mods.into(), 16416);

  let mods = Mods::Key2;
  eq(mods.into(), 268435456);
}
