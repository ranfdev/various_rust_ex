use clap::Parser;

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžź
ż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzz
z";
fn conv(c: char) -> char {
    if let Some(i) = SUBS_I.chars().position(|x| x == c) {
        SUBS_O.chars().nth(i).unwrap()
    } else if !c.is_ascii_alphanumeric() {
        '-'
    } else {
        c
    }
}
fn slugify(s: &str) -> String {
    // first we convert the single characters
    let iter_normalized_chars = s.chars().flat_map(|c| c.to_lowercase()).map(|c| conv(c));

    // then we apply the rules on how to construct the string
    // - the string cannot end in '-', unless it's the only char
    // - consecutive '-' are dropped
    let mut last_checked_is_dash = false;
    let string = iter_normalized_chars.fold(String::new(), |mut acc, el| {
        dbg!(last_checked_is_dash);
        if el == '-' {
            if !last_checked_is_dash {
                acc.push(el);
            };
            last_checked_is_dash = true;
            acc
        } else {
            last_checked_is_dash = false;
            acc.push(el);
            acc
        }
    });
    if string.len() == 1 {
        string
    } else {
        string.trim_end_matches('-').to_string()
    }
}

#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
    #[arg(long)]
    repeat: usize,
    #[arg(long)]
    verbose: bool,
}
fn main() {
    let args = Args::parse();
    dbg!(slugify(&args.slug_in));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_char_accent() {
        assert_eq!(conv('à'), 'a');
    }

    #[test]
    fn convert_char_no_accent() {
        assert_eq!(conv('a'), 'a');
    }

    #[test]
    fn convert_unknown_char() {
        assert_eq!(conv(')'), '-');
    }

    #[test]
    fn convert_unknown_accent() {
        assert_eq!(conv('ῶ'), '-');
    }

    #[test]
    fn string_with_space() {
        assert_eq!(slugify("guido marconi"), "guido-marconi");
    }

    #[test]
    fn string_with_accents() {
        assert_eq!(slugify("ëēėęě"), "eeeee");
    }

    #[test]
    fn empty_string() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn many_consecutive_non_valid() {
        assert_eq!(slugify("$%56&^^^78"), "-56-78");
    }

    #[test]
    fn only_non_valid() {
        assert_eq!(slugify("$%&^^^"), "-");
    }

    #[test]
    fn space_at_end() {
        assert_eq!(slugify("ciao "), "ciao");
    }
    #[test]
    fn many_non_valid_at_end() {
        assert_eq!(slugify("123%^&"), "123");
    }
}
