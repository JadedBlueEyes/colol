pub mod colors;

#[macro_export]
macro_rules! color {
  ($name: ident) => (
    print!("\u{1B}[{}m", $crate::colors::COLORS.$name[0]);
  )
}

#[macro_export]
macro_rules! close_color {
  ($name: ident) => (
    print!("\u{1B}[{}m", $crate::colors::COLORS.$name[1]);
  )
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        color!(blue);
        println!("Test basic!");
        color!(reset);
    }

    #[test]
    fn closeing_colors() {
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
}
