use pyo3::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use rsmorphy::prelude::*;
use rayon::prelude::*;
use regex::{Regex};
use rayon::{ThreadPoolBuilder, ThreadPool};

#[pyclass]
struct RSMorphyLemmatizer {
    stop_words: HashSet<String>,
    name_token: String,
    surname_token: String,
    patronymic_token: String,
    morph: MorphAnalyzer,
    replace_fio: bool,
    split_regex: Regex,
    name_gr: Grammeme,
    surname_gr: Grammeme,
    patr_gr: Grammeme,
    executor: Arc<Mutex<Option<ThreadPool>>>
}

#[pymethods]
impl RSMorphyLemmatizer {
    #[new]
    #[pyo3(signature=(dict_path, stop_words, name_token="[Name]", surname_token="[Surn]",
    patronymic_token="[Patr]", replace_fio=true, n_jobs=1, split_re="\\s"))]
    pub fn new(dict_path: &str, stop_words: HashSet<String>, name_token: &str,
               surname_token: &str, patronymic_token: &str, replace_fio: bool,
               n_jobs: usize, split_re: &str) -> RSMorphyLemmatizer {
        let morph = MorphAnalyzer::from_file(dict_path);
        let split_regex = Regex::new(&split_re).expect("Error compile split regex");
        let name_gr = Grammeme::new("Name");
        let surname_gr = Grammeme::new("Surn");
        let patr_gr = Grammeme::new("Patr");
        let executor;
        if n_jobs > 1 {
            executor = Arc::new(Mutex::new(Some(ThreadPoolBuilder::new().num_threads(n_jobs).build().unwrap())));
        } else {
            executor = Arc::new(Mutex::new(None));
        }
        RSMorphyLemmatizer { stop_words, name_token: name_token.to_string(),
            surname_token: surname_token.to_string(), patronymic_token: patronymic_token.to_string(),
            morph, replace_fio, split_regex, name_gr, surname_gr, patr_gr, executor }
    }

    #[pyo3(signature=())]
    pub fn to_single_thread(&mut self) {
        let mut e = self.executor.lock().unwrap();
        *e = None;
    }

    #[pyo3(signature=(text_list))]
    pub fn transform(&mut self, text_list: Vec<String>) -> Vec<String> {
        match self.executor.lock().expect("Can't lock mutex").as_ref() {
            Some(e) => {
                e.install(|| {
                    text_list.into_par_iter().map(|text| RSMorphyLemmatizer::process_text(
                        &text, &self.split_regex, &self.morph, self.replace_fio,
                        &self.name_token, &self.surname_token, &self.patronymic_token,
                        &self.name_gr, &self.surname_gr, &self.patr_gr, &self.stop_words
                    )).collect::<Vec<String>>()
                })
            },
            None => {
                text_list.iter().map(|text| RSMorphyLemmatizer::process_text(
                    text, &self.split_regex, &self.morph, self.replace_fio,
                    &self.name_token, &self.surname_token, &self.patronymic_token,
                    &self.name_gr, &self.surname_gr, &self.patr_gr, &self.stop_words
                )).collect::<Vec<String>>()
            }
        }
    }
}

impl RSMorphyLemmatizer {

    fn process_text(text: &str, split_regex: &Regex, morph: &MorphAnalyzer,
                    fio_replace: bool, name_token: &str, surname_token: &str, patr_token: &str,
                    name_gr: &Grammeme, surname_gr: &Grammeme, patr_gr: &Grammeme, stop_words: &HashSet<String>) -> String {
        split_regex.split(text).into_iter().map(|word| RSMorphyLemmatizer::process_word(
            word, morph, fio_replace, name_token, surname_token, patr_token, name_gr, surname_gr, patr_gr
        )).filter(|v| !stop_words.contains(v)).collect::<Vec<String>>().join(" ")
    }

    fn process_word(word: &str, morph: &MorphAnalyzer, fio_replace: bool,
                    name_token: &str, surname_token: &str, patr_token: &str,
                    name_gr: &Grammeme, surname_gr: &Grammeme, patr_gr: &Grammeme) -> String {
        match morph.parse(word).first() {
            Some(p) => {
                if fio_replace {
                    if p.lex.get_tag(&morph).grammemes.set.contains(name_gr) {
                        return name_token.to_string();
                    }
                    if p.lex.get_tag(&morph).grammemes.set.contains(surname_gr) {
                        return surname_token.to_string();
                    }
                    if p.lex.get_tag(&morph).grammemes.set.contains(patr_gr) {
                        return patr_token.to_string();
                    }
                }
                p.lex.get_lemma(&morph).get_word().to_string()
            },
            None => "[PARSE_ERROR]".to_string()
        }
    }
}

#[pymodule]
fn rsmorphy_lemmatizer(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RSMorphyLemmatizer>()?;
    Ok(())
}