use std::collections::{ HashSet, HashMap };
use std::iter::FromIterator;
use crate::structure::*;
extern crate reqwest;
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
#[derive(Debug, Clone)]
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

fn find_parts(json :&str)->std::result::Result<Parts, ()> {
    let mut ret = Parts::new();
    let mut from = 0;
    while let Some(i) = json[from..].find("\"fl\"") {
        let end = json[from + i + 6..].find('"').unwrap();
        let part = Part::from_string(&json[from + i + 6..from + i + 6 + end]);
        from += i + 6 + end;
        ret.insert(part);
    }
    if ret.is_empty() {
        ret.insert(Part::N);
    }
    return Ok(ret);
}

impl<'w> Word<'w> {
    pub fn get_info(word :&'w str)->std::result::Result<Parts, ()> {
        match word {
            "is" | "was" | "were" | "be" | "been" | "am" => {
                let mut a = Part::Prog.to_multi();
                a.insert(Part::V);
                Ok(a)
            },
            "very" | "extremely" => Ok(Part::Deg.to_multi()),
            _ => {
                let url = format!("https://www.dictionaryapi.com/api/v3/references/collegiate/json/{}?key=e235db78-58c1-42ae-b081-1fc9cfb65810", word.to_ascii_lowercase());
                let res = reqwest::get(&url).ok().ok_or(())?.text().ok().ok_or(())?;
                return find_parts(&res[..]);
            }
        }
    }
    pub fn new(word :&'w str)->Word<'w> {
        let mut cache = WordCache::cache();
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
            Word {
                word: word,
                part: part
            }
        }
    }
    pub fn is_part(&self, part :Part)->bool {
        self.part.contains(&part)
    }
    pub fn determine(&self, part :Part)->Option<DeterminedWord<'w>> {
        if self.part.contains(&part) {
            return Some(DeterminedWord {
                word: self.word,
                part: part
            });
        }
        return None;
    }
}

pub fn sentence_to_vec<'w>(s :&'w str)->Vec<Word<'w>> {
    let mut ret = Vec::new();
    for word in s.split(' ') {
        ret.push(Word::new(word));
    }
    return ret;
}