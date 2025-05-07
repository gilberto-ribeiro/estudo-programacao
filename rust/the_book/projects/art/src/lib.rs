//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    //! This module has the kinds of colors.

    /// The primary colors according to the RYB color model.
    #[derive(PartialEq, Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(PartialEq, Debug)]
    pub enum SecondaryColor {
        Orange,
        Purple,
        Green,
    }
}

pub mod utils {
    //! This module has the functions.

    use std::result::Result;

    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///
    /// # Example
    /// ```
    /// let color_1 = art::PrimaryColor::Blue;
    /// let color_2 = art::PrimaryColor::Yellow;
    /// let mixed_color = art::SecondaryColor::Green;
    ///
    /// assert_eq!(mixed_color, art::mix(&color_1, &color_2).unwrap());
    /// assert_eq!(mixed_color, art::mix(&color_2, &color_1).unwrap());
    /// ```
    pub fn mix(
        color_1: &PrimaryColor,
        color_2: &PrimaryColor,
    ) -> Result<SecondaryColor, &'static str> {
        let error_message = "Wrong combination of colors.";
        match color_1 {
            PrimaryColor::Red => match color_2 {
                PrimaryColor::Yellow => Ok(SecondaryColor::Orange),
                PrimaryColor::Blue => Ok(SecondaryColor::Purple),
                _ => Err(error_message),
            },
            PrimaryColor::Yellow => match color_2 {
                PrimaryColor::Red => Ok(SecondaryColor::Orange),
                PrimaryColor::Blue => Ok(SecondaryColor::Green),
                _ => Err(error_message),
            },
            PrimaryColor::Blue => match color_2 {
                PrimaryColor::Red => Ok(SecondaryColor::Purple),
                PrimaryColor::Yellow => Ok(SecondaryColor::Green),
                _ => Err(error_message),
            },
        }
    }
}
