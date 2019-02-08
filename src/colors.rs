//! See `const COLORS` for a list of all the colors.
//! Other than that, his module contains internals.

/// Generates the struct definition for `Colors`.
macro_rules! make_defs {
  ($($name: ident),+) => (
  /// A struct containing all the different colors.
  /// Should not be constructed or matched against,
  /// as any changes to this are not considered to be breaking.
  /// See `const COLORS` for a list of all the colors.
  pub struct Colors {
    $(
      pub $name: [u8; 2],
    )+
  }
  )
}

// #[non_exhaustive]
make_defs!(
    reset,
    bold,
    dim,
    italic,
    underline,
    inverse,
    hidden,
    strikethrough,
    black,
    red,
    green,
    yellow,
    blue,
    magenta,
    cyan,
    white,
    gray,
    grey,
    lt_black,
    lt_red,
    lt_green,
    lt_yellow,
    lt_blue,
    lt_magenta,
    lt_cyan,
    lt_white,
    bg_black,
    bg_red,
    bg_green,
    bg_yellow,
    bg_blue,
    bg_magenta,
    bg_cyan,
    bg_white,
    bg_lt_black,
    bg_lt_red,
    bg_lt_green,
    bg_lt_yellow,
    bg_lt_blue,
    bg_lt_magenta,
    bg_lt_cyan,
    bg_lt_white
);

/// The struct containing the integers for the colors.
/// Should not be accessed directly, as will be used by
/// the `color!` and `close_color!` macros.
/// # Values
///
/// ## Formatting
///
/// - `reset`: Resets the formatting of the terminal to default
/// - `bold`: Formats text to bold.
/// - `dim`: Formats text to be dim.
/// - `italic`: Formats text to be italic.
/// - `underline`: Formats text to be underlined.
/// - `inverse`: Swaps foreground and background colors of the
///     terminal.
/// - `hidden`: Is supposed to hide the text,
///     however does not on some platforms, and behaviour varies.
/// - `strikethrough`: Is supposed to format text with a strikethrough,
///     however does not in some platforms.
///
/// ## Foreground Colors
///
/// - `black`: Makes text black.
/// - `red`: Makes text red.
/// - `green`: Makes text green.
/// - `yellow`: Makes text yellow.
/// - `blue`: Makes text blue.
/// - `magenta`: Makes text magenta.
/// - `cyan`: Makes text cyan.
/// - `white`: Makes text white.
/// - `gray`: Makes text gray. Same light black.
/// - `grey`: Makes text grey. Same light black.
/// - `lt_black`: Makes text light black.
/// - `lt_red`: Makes text light red.
/// - `lt_green`: Makes text light green.
/// - `lt_yellow`: Makes text light yellow.
/// - `lt_blue`: Makes text light blue.
/// - `lt_magenta`: Makes text light magenta.
/// - `lt_cyan`: Makes text light cyan.
/// - `lt_white`: Makes text light white.
///
/// ## Background Colors
/// - `bg_black`: makes the background black.
/// - `bg_red`: makes the background red.
/// - `bg_green`: makes the background green.
/// - `bg_yellow`: makes the background yellow.
/// - `bg_blue`: makes the background blue.
/// - `bg_magenta`: makes the background magenta.
/// - `bg_cyan`: makes the background cyan.
/// - `bg_white`: makes the background white.
/// - `bg_lt_black`: makes the background light black.
/// - `bg_lt_red`: makes the background light red.
/// - `bg_lt_green`: makes the background light green.
/// - `bg_lt_yellow`: makes the background light yellow.
/// - `bg_lt_blue`: makes the background light blue.
/// - `bg_lt_magenta`: makes the background light magenta.
/// - `bg_lt_cyan`: makes the background light cyan.
/// - `bg_lt_white`: makes the background light white.
pub const COLORS: Colors = Colors {
    reset: [0, 0],

    bold: [1, 22],
    dim: [2, 22],
    italic: [3, 23],
    underline: [4, 24],
    inverse: [7, 27],
    hidden: [8, 28],
    strikethrough: [9, 29],

    black: [30, 39],
    red: [31, 39],
    green: [32, 39],
    yellow: [33, 39],
    blue: [34, 39],
    magenta: [35, 39],
    cyan: [36, 39],
    white: [37, 39],
    gray: [90, 39],
    grey: [90, 39],

    lt_black: [90, 39],
    lt_red: [91, 39],
    lt_green: [92, 39],
    lt_yellow: [93, 39],
    lt_blue: [94, 39],
    lt_magenta: [95, 39],
    lt_cyan: [96, 39],
    lt_white: [97, 39],

    bg_black: [40, 49],
    bg_red: [41, 49],
    bg_green: [42, 49],
    bg_yellow: [43, 49],
    bg_blue: [44, 49],
    bg_magenta: [45, 49],
    bg_cyan: [46, 49],
    bg_white: [47, 49],

    bg_lt_black: [100, 49],
    bg_lt_red: [101, 49],
    bg_lt_green: [102, 49],
    bg_lt_yellow: [103, 49],
    bg_lt_blue: [104, 49],
    bg_lt_magenta: [105, 49],
    bg_lt_cyan: [106, 49],
    bg_lt_white: [107, 49],
};
