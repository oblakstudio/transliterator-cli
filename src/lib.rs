//! Serbian Cyrillic ↔ Latin transliteration.
//!
//! Port of <https://github.com/oblakstudio/transliterator>.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Cyrillic → Latin replacement pairs (with diacritics).
const CIR_TO_LAT_PAIRS: &[(&str, &str)] = &[
    ("А", "A"), ("Б", "B"), ("В", "V"), ("Г", "G"), ("Д", "D"),
    ("Ђ", "Đ"), ("Е", "E"), ("Ж", "Ž"), ("З", "Z"), ("И", "I"),
    ("Ј", "J"), ("К", "K"), ("Л", "L"), ("Љ", "LJ"), ("М", "M"),
    ("Н", "N"), ("Њ", "NJ"), ("О", "O"), ("П", "P"), ("Р", "R"),
    ("С", "S"), ("Т", "T"), ("Ћ", "Ć"), ("У", "U"), ("Ф", "F"),
    ("Х", "H"), ("Ц", "C"), ("Ч", "Č"), ("Џ", "DŽ"), ("Ш", "Š"),
    ("а", "a"), ("б", "b"), ("в", "v"), ("г", "g"), ("д", "d"),
    ("ђ", "đ"), ("е", "e"), ("ж", "ž"), ("з", "z"), ("и", "i"),
    ("ј", "j"), ("к", "k"), ("л", "l"), ("љ", "lj"), ("м", "m"),
    ("н", "n"), ("њ", "nj"), ("о", "o"), ("п", "p"), ("р", "r"),
    ("с", "s"), ("т", "t"), ("ћ", "ć"), ("у", "u"), ("ф", "f"),
    ("х", "h"), ("ц", "c"), ("ч", "č"), ("џ", "dž"), ("ш", "š"),
    ("Ња", "Nja"), ("Ње", "Nje"), ("Њи", "Nji"), ("Њо", "Njo"), ("Њу", "Nju"),
    ("Ља", "Lja"), ("Ље", "Lje"), ("Љи", "Lji"), ("Љо", "Ljo"), ("Љу", "Lju"),
    ("Џа", "Dža"), ("Џе", "Dže"), ("Џи", "Dži"), ("Џо", "Džo"), ("Џу", "Džu"),
];

/// Cyrillic → Latin overrides for the diacritic-free ("cut") variant.
const CIR_TO_CUT_OVERRIDES: &[(&str, &str)] = &[
    ("Ђ", "Dj"), ("Ж", "Z"), ("Ч", "C"), ("Ћ", "C"), ("Џ", "Dz"), ("Ш", "S"),
    ("ђ", "dj"), ("ж", "z"), ("ч", "c"), ("ћ", "c"), ("џ", "dz"), ("ш", "s"),
    ("Џа", "Dza"), ("Џе", "Dze"), ("Џи", "Dzi"), ("Џо", "Dzo"), ("Џу", "Dzu"),
];

/// Latin (with diacritics) → diacritic-free Latin.
const LAT_TO_CUT_PAIRS: &[(&str, &str)] = &[
    ("DŽ", "Dz"), ("Dž", "Dz"), ("dž", "dz"),
    ("Č", "C"), ("č", "c"),
    ("Ć", "C"), ("ć", "c"),
    ("Đ", "Dj"), ("đ", "dj"),
    ("Š", "S"), ("š", "s"),
    ("Ž", "Z"), ("ž", "z"),
];

/// A transliteration table. Caches the maximum key length (in `char`s) so
/// scanning can try longest matches first without re-measuring.
pub struct Table {
    map: HashMap<String, &'static str>,
    max_key_chars: usize,
}

impl Table {
    fn from_pairs(pairs: &[(&'static str, &'static str)]) -> Self {
        let mut map = HashMap::with_capacity(pairs.len());
        let mut max_key_chars = 0;
        for (k, v) in pairs {
            let len = k.chars().count();
            if len > max_key_chars {
                max_key_chars = len;
            }
            map.insert((*k).to_string(), *v);
        }
        Self { map, max_key_chars }
    }

    fn merge(base: &[(&'static str, &'static str)], overrides: &[(&'static str, &'static str)]) -> Self {
        let mut t = Self::from_pairs(base);
        for (k, v) in overrides {
            let len = k.chars().count();
            if len > t.max_key_chars {
                t.max_key_chars = len;
            }
            t.map.insert((*k).to_string(), *v);
        }
        t
    }

    fn flipped(pairs: &[(&'static str, &'static str)]) -> Self {
        let mut map = HashMap::with_capacity(pairs.len());
        let mut max_key_chars = 0;
        for (k, v) in pairs {
            let len = v.chars().count();
            if len > max_key_chars {
                max_key_chars = len;
            }
            map.insert((*v).to_string(), *k);
        }
        Self { map, max_key_chars }
    }

    /// Greedy longest-match transliteration.
    pub fn translit(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut i = 0;
        let mut buf = String::new();
        while i < chars.len() {
            let max_try = self.max_key_chars.min(chars.len() - i);
            let mut matched = false;
            for len in (1..=max_try).rev() {
                buf.clear();
                buf.extend(&chars[i..i + len]);
                if let Some(repl) = self.map.get(&buf) {
                    out.push_str(repl);
                    i += len;
                    matched = true;
                    break;
                }
            }
            if !matched {
                out.push(chars[i]);
                i += 1;
            }
        }
        out
    }
}

fn cir_to_lat_table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| Table::from_pairs(CIR_TO_LAT_PAIRS))
}

fn cir_to_cut_lat_table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| Table::merge(CIR_TO_LAT_PAIRS, CIR_TO_CUT_OVERRIDES))
}

fn lat_to_cir_table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| Table::flipped(CIR_TO_LAT_PAIRS))
}

fn lat_to_cut_lat_table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| Table::from_pairs(LAT_TO_CUT_PAIRS))
}

/// Cyrillic → Latin (with diacritics).
pub fn cir_to_lat(text: &str) -> String {
    cir_to_lat_table().translit(text)
}

/// Cyrillic → diacritic-free Latin.
pub fn cir_to_cut_lat(text: &str) -> String {
    cir_to_cut_lat_table().translit(text)
}

/// Latin (with diacritics) → Cyrillic.
pub fn lat_to_cir(text: &str) -> String {
    lat_to_cir_table().translit(text)
}

/// Latin (with diacritics) → diacritic-free Latin.
pub fn lat_to_cut_lat(text: &str) -> String {
    lat_to_cut_lat_table().translit(text)
}

/// The four supported transliteration directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    CirToLat,
    CirToCutLat,
    LatToCir,
    LatToCutLat,
}

impl Mode {
    pub fn apply(self, text: &str) -> String {
        match self {
            Mode::CirToLat => cir_to_lat(text),
            Mode::CirToCutLat => cir_to_cut_lat(text),
            Mode::LatToCir => lat_to_cir(text),
            Mode::LatToCutLat => lat_to_cut_lat(text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cir_to_lat_readme_example() {
        assert_eq!(
            cir_to_lat("Четири Чавчића риби гризу реп!"),
            "Četiri Čavčića ribi grizu rep!"
        );
    }

    #[test]
    fn cir_to_cut_lat_readme_example() {
        assert_eq!(
            cir_to_cut_lat("Четири Чавчића риби гризу реп!"),
            "Cetiri Cavcica ribi grizu rep!"
        );
    }

    #[test]
    fn lat_to_cir_readme_example() {
        assert_eq!(
            lat_to_cir("Da l' si ikada mene voljela?"),
            "Да л' си икада мене вољела?"
        );
    }

    #[test]
    fn lat_to_cut_lat_strips_diacritics() {
        assert_eq!(lat_to_cut_lat("Četiri Čavčića"), "Cetiri Cavcica");
        assert_eq!(lat_to_cut_lat("Đorđe Džaja"), "Djordje Dzaja");
    }

    #[test]
    fn cir_to_lat_handles_compound_njegos() {
        // "Њу" is a 2-char digraph entry, must beat single-char "Њ" + "у".
        assert_eq!(cir_to_lat("Њујорк"), "Njujork");
        assert_eq!(cir_to_lat("Љубав"), "Ljubav");
        assert_eq!(cir_to_lat("Џабе"), "Džabe");
    }

    #[test]
    fn cir_to_cut_lat_handles_digraph_overrides() {
        // "Џе" -> "Dze" must win over "Џ"->"Dz" + "е"->"e" (same here, but
        // explicit table entry guards regression).
        assert_eq!(cir_to_cut_lat("Џек"), "Dzek");
    }

    #[test]
    fn empty_input_is_empty_output() {
        assert_eq!(cir_to_lat(""), "");
        assert_eq!(lat_to_cir(""), "");
    }

    #[test]
    fn ascii_passthrough() {
        assert_eq!(cir_to_lat("hello, 123!"), "hello, 123!");
    }

    #[test]
    fn mode_apply_dispatches() {
        assert_eq!(
            Mode::CirToLat.apply("Београд"),
            "Beograd"
        );
    }
}
