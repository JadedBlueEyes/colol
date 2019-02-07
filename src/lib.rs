pub mod colors;

#[cfg(feature = "colol")]
#[macro_export]
macro_rules! color {
    ($name: ident) => {
        print!("\u{1B}[{}m", $crate::colors::COLORS.$name[0]);
    };
}

#[cfg(feature = "colol")]
#[macro_export]
macro_rules! close_color {
    ($name: ident) => {
        print!("\u{1B}[{}m", $crate::colors::COLORS.$name[1]);
    };
}

#[cfg(not(feature = "colol"))]
#[macro_export]
macro_rules! color {
    ($name: ident) => {};
}

#[cfg(not(feature = "colol"))]
#[macro_export]
macro_rules! close_color {
    ($name: ident) => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_enabled() {
        assert_eq!(true, cfg!(feature = "colol"));
    }

    #[test]
    fn basic() {
        color!(blue);
        println!("Test basic!");
        color!(reset);
    }

    #[test]
    fn closing_colors() {
        color!(blue);
        println!("Test closing colors!");
        close_color!(blue);
    }

    #[test]
    fn multiple_colors() {
        color!(blue);
        print!("Test ");
        close_color!(blue);
        color!(red);
        color!(underline);
        print!("multiple");
        close_color!(underline);
        println!(" colors!");
        close_color!(red);
    }

    #[test]
    fn logo() {
        color!(bold);
        color!(bg_red);
        color!(white);
        print!("co");
        close_color!(bg_red);
        color!(bg_blue);
        println!("lol");
        close_color!(bg_blue);
        close_color!(bold);
    }

    #[test]
    fn every_color() {
        color!(bold);
        println!("bold");
        close_color!(bold);

        color!(dim);
        println!("dim");
        close_color!(dim);

        color!(italic);
        println!("italic");
        close_color!(italic);

        color!(underline);
        println!("underline");
        close_color!(underline);

        color!(inverse);
        println!("inverse");
        close_color!(inverse);

        color!(hidden);
        println!("hidden");
        close_color!(hidden);

        color!(strikethrough);
        println!("strikethrough");
        close_color!(strikethrough);

        color!(black);
        println!("black");
        close_color!(black);

        color!(red);
        println!("red");
        close_color!(red);

        color!(green);
        println!("green");
        close_color!(green);

        color!(yellow);
        println!("yellow");
        close_color!(yellow);

        color!(blue);
        println!("blue");
        close_color!(blue);

        color!(magenta);
        println!("magenta");
        close_color!(magenta);

        color!(cyan);
        println!("cyan");
        close_color!(cyan);

        color!(white);
        println!("white");
        close_color!(white);

        color!(gray);
        println!("gray");
        close_color!(gray);

        color!(grey);
        println!("grey");
        close_color!(grey);

        color!(lt_black);
        println!("lt_black");
        close_color!(lt_black);

        color!(lt_red);
        println!("lt_red");
        close_color!(lt_red);

        color!(lt_green);
        println!("lt_green");
        close_color!(lt_green);

        color!(lt_yellow);
        println!("lt_yellow");
        close_color!(lt_yellow);

        color!(lt_blue);
        println!("lt_blue");
        close_color!(lt_blue);

        color!(lt_magenta);
        println!("lt_magenta");
        close_color!(lt_magenta);

        color!(lt_cyan);
        println!("lt_cyan");
        close_color!(lt_cyan);

        color!(lt_white);
        println!("lt_white");
        close_color!(lt_white);

        color!(bg_black);
        println!("bg_black");
        close_color!(bg_black);

        color!(bg_red);
        println!("bg_red");
        close_color!(bg_red);

        color!(bg_green);
        println!("bg_green");
        close_color!(bg_green);

        color!(bg_yellow);
        println!("bg_yellow");
        close_color!(bg_yellow);

        color!(bg_blue);
        println!("bg_blue");
        close_color!(bg_blue);

        color!(bg_magenta);
        println!("bg_magenta");
        close_color!(bg_magenta);

        color!(bg_cyan);
        println!("bg_cyan");
        close_color!(bg_cyan);

        color!(bg_white);
        println!("bg_white");
        close_color!(bg_white);

        color!(bg_lt_black);
        println!("bg_lt_black");
        close_color!(bg_lt_black);

        color!(bg_lt_red);
        println!("bg_lt_red");
        close_color!(bg_lt_red);

        color!(bg_lt_green);
        println!("bg_lt_green");
        close_color!(bg_lt_green);

        color!(bg_lt_yellow);
        println!("bg_lt_yellow");
        close_color!(bg_lt_yellow);

        color!(bg_lt_blue);
        println!("bg_lt_blue");
        close_color!(bg_lt_blue);

        color!(bg_lt_magenta);
        println!("bg_lt_magenta");
        close_color!(bg_lt_magenta);

        color!(bg_lt_cyan);
        println!("bg_lt_cyan");
        close_color!(bg_lt_cyan);

        color!(bg_lt_white);
        println!("bg_lt_white");
        close_color!(bg_lt_white);
    }
}
