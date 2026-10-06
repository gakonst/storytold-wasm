use serde_derive::{Deserialize, Serialize};

/// Aspect ratios a model can be asked for. Not every model supports every
/// value; a model's config lists its own.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommonAspectRatio {
  /// Let the model decide (e.g. follow the source image when editing).
  Auto,
  Square,
  WideThreeByTwo,
  WideFourByThree,
  WideFiveByFour,
  WideSixteenByNine,
  WideTwentyOneByNine,
  TallTwoByThree,
  TallThreeByFour,
  TallFourByFive,
  TallNineBySixteen,
  TallNineByTwentyOne,
  /// Imprecise semantic values some models map to their own sizes.
  Wide,
  Tall,
  /// Auto aspect at a baked-in resolution.
  #[serde(rename = "auto_2k")]
  Auto2k,
  #[serde(rename = "auto_3k")]
  Auto3k,
  #[serde(rename = "auto_4k")]
  Auto4k,
  /// Square at a baked-in HD resolution.
  SquareHd,
}
