use std::error::Error;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Highlighter {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let syntax_set = SyntaxSet::load_defaults_nonewlines();
        let theme_set = ThemeSet::load_defaults();
        Ok(Highlighter {
            syntax_set,
            theme_set,
        })
    }

    pub fn highlight(&self, input: &str) -> Result<(), Box<dyn Error>> {
        let syntax = self.syntax_set.find_syntax_by_extension("js").unwrap();
        let mut highlighter =
            HighlightLines::new(syntax, &self.theme_set.themes["Solarized (light)"]);

        for line in LinesWithEndings::from(input) {
            let regions = highlighter.highlight_line(line, &self.syntax_set)?;
            let highlighted = as_24_bit_terminal_escaped(&regions[..], false);
            println!("{}", highlighted);
        }

        Ok(())
    }
}
