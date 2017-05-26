use regex::{escape, Regex, Captures};

pub struct Inflector {
    acronyms: Vec<String>,
    word_regex: Option<Regex>,
    adjacent_caps_regex: Regex,
    adjacent_regex: Regex,
}

fn word_regex_from_acronyms(acronyms: &[String]) -> Regex {
    let mut re = r"(?:([A-Za-z\d])|^)(".to_owned();

    for (idx, acr) in acronyms.iter().enumerate() {
        if idx > 0 {
            re += "|";
        }

        re += &escape(acr);
    }

    re += r")\b";

    Regex::new(&re).unwrap()
}

impl Inflector {
    pub fn new() -> Inflector {
        Inflector {
            acronyms: vec![],
            word_regex: None,
            adjacent_caps_regex: Regex::new(r"([A-Z\d]+)([A-Z][a-z])").unwrap(),
            adjacent_regex: Regex::new(r"([a-z\d])([A-Z])").unwrap(),
        }
    }

    pub fn add_acronym(&mut self, acronym: String) {
        self.acronyms.push(acronym);
        self.word_regex = Some(word_regex_from_acronyms(&self.acronyms));
    }

    pub fn underscore(&self, s: &str) -> String {
        let s = s.replace("::", "/");

        let s = self.word_regex.as_ref().map(|re| re.replace_all(&s, |caps: &Captures| {
            let mut replacement = String::new();

            if let Some(c) = caps.get(1) {
                replacement += c.as_str();
                replacement += "_";
            }

            replacement += &caps[2].to_lowercase();

            replacement
        }).into_owned()).unwrap_or(s);

        let s = self.adjacent_caps_regex.replace_all(&s, "${1}_${2}");

        let s = self.adjacent_regex.replace_all(&s, "${1}_${2}");

        let s = s.replace("-", "_");

        s.to_lowercase()
    }
}
