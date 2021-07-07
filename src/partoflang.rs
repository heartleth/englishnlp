use std::collections::{ HashSet, HashMap };
use crate::structure::*;
extern crate reqwest;
extern crate serde_json;
use serde_json::{ Value };
use std::io::Write;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct WordCache {
    hash: HashMap<String, Part>
}
pub type Parts = HashSet<Part>;
#[derive(Debug)]
pub struct Word<'w> {
    pub word: &'w str,
    pub part: Part,
    pub cache: WordCache
}

impl<'w> fmt::Display for Word<'w> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl WordCache {
    pub fn cache()->WordCache {
        if let Ok(s) = fs::read_to_string("dictcache.txt") {
            let s :Vec<&str> = s.split('\n').collect();
            let mut ret = HashMap::with_capacity(s.len());
            for e in s {
                if e.len() > 0 {
                    let i :Vec<&str> = e.split('/').collect();
                    ret.insert(i[0].to_string(), Part::from_string(i[1]));
                }
            }
            WordCache {
                hash: ret
            }
        }
        else {
            fs::write("dictcache.txt", "").unwrap();
            WordCache {
                hash: HashMap::new()
            }
        }
    }
    pub fn get(&self, word :&str)->Option<Part> {
        self.hash.get(word).and_then(|p|Some(*p))
    }
    pub fn register(&mut self, word :&str, p :Part) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("dictcache.txt")
            .unwrap();
        writeln!(file, "{}/{:?}", word, p).unwrap();
        self.hash.insert(word.to_string(), p);
    }
}

impl<'w> Word<'w> {
    pub fn get_info(word :&'w str)->std::result::Result<Part, ()> {
        match word {
            "is" | "was" | "were" | "be" | "am" => Ok(Part::Aux),
            _ => {
                let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key=e235db78-58c1-42ae-b081-1fc9cfb65810", word);
                let res = reqwest::get(&url).ok().ok_or(())?.text().ok().ok_or(())?;
                let res :Value = serde_json::from_str(&res[..]).ok().ok_or(())?;
                let info = &res[0];
                Ok(Part::from_string(&info["fl"].as_str().ok_or(())?))
            }
        }
    }
    pub fn new(word :&'w str)->Word<'w> {
        let mut cache = WordCache::cache();
        if let Some(part) = cache.get(word) {
            Word {
                word: word,
                part: part,
                cache: cache
            }
        }
        else {
            let part = Word::get_info(word).unwrap();
            cache.register(word, part);
            Word {
                word: word,
                part: part,
                cache: cache
            }
        }
    }
}

pub fn sentence_to_vec<'w>(s :&'w str)->Vec<Word<'w>> {
    let mut ret = Vec::new();
    for word in s.split(' ') {
        ret.push(Word::new(word));
    }
    return ret;
}