use std::collections::{ HashSet, HashMap };
use std::iter::FromIterator;
use crate::structure::*;
extern crate reqwest;
extern crate serde_json;
use serde_json::{ Value };
use std::io::Write;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct WordCache {
    hash: HashMap<String, Parts>
}
pub type Parts = HashSet<Part>;

#[derive(Debug)]
pub struct Word<'w> {
    pub word: &'w str,
    pub part: Parts
}
#[derive(Debug)]
pub struct DeterminedWord<'w> {
    pub word: &'w str,
    pub part: Part
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
                    let parts = i[1].split(',');
                    let parts :HashSet<Part> = HashSet::from_iter(parts.filter(|x|x.len()>0).map(|x|Part::from_string(x)));
                    ret.insert(i[0].to_string(), parts);
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
    pub fn has(&self, word :&str)->bool {
        self.hash.contains_key(&word.to_ascii_lowercase()[..])
    }
    pub fn get_into(mut self, word :&str)->Parts {
        self.hash.remove(&word.to_ascii_lowercase()[..]).unwrap()
    }
    pub fn register(&mut self, word :&str, p :&Parts) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("dictcache.txt")
            .unwrap();
        let to_write = p.iter().fold(String::new(), |a, b| a + &format!("{:?}", b)[..] + ",");
        writeln!(file, "{}/{}", word.to_ascii_lowercase(), to_write).unwrap();
    }
}

impl<'w> Word<'w> {
    pub fn get_info(word :&'w str)->std::result::Result<Parts, ()> {
        println!("Searching for the word \"{}\"...", word);
        match word {
            "is" | "was" | "were" | "be" | "am" => {
                let mut a = Part::Deg.to_multi();
                a.insert(Part::V);
                Ok(a)
            },
            "very" | "extremely" => Ok(Part::Deg.to_multi()),
            _ => {
                let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key=e235db78-58c1-42ae-b081-1fc9cfb65810", word.to_ascii_lowercase());
                let res = reqwest::get(&url).ok().ok_or(())?.text().ok().ok_or(())?;
                let res :Value = serde_json::from_str(&res[..]).ok().ok_or(())?;
                let mut ret = Parts::new();
                for info in res.as_array().ok_or(())? {
                    if let Some(p) = &info["fl"].as_str() {
                        let mut part = Part::from_string(p);
                        if part == Part::V {
                            if info["def"].as_array().ok_or(())?.iter()
                                .any(|a| a["vd"].as_str() == Some("auxiliary verb")) {

                                part = Part::Aux;
                            }
                        }
                        ret.insert(part);
                    }
                }
                Ok(ret)
            }
        }
    }
    pub fn new(word :&'w str)->Word<'w> {
        let mut cache = WordCache::cache();
        // if let Some(part) = cache.(word) {
        if cache.has(&word.to_ascii_lowercase()[..]) {
            let cache_into = cache;
            let part = cache_into.get_into(word);
            Word {
                word: word,
                part: part
            }
        }
        else {
            let part = Word::get_info(word).unwrap();
            cache.register(word, &part);
            // let own
            Word {
                word: word,
                part: part
            }
        }
    }
    pub fn is_part(&self, part :Part)->bool {
        self.part.contains(&part)
    }
    pub fn determine(&self, part :Part)->DeterminedWord<'w> {
        DeterminedWord {
            word: self.word,
            part: part
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