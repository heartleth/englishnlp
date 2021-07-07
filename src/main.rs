#[macro_use]
mod structure;
use structure::*;

mod phrase;
mod partoflang;
use partoflang::*;

fn main() {
    let grammer = grammer!(
        grammer_s!(S -> NP VP)
        grammer_s!(S -> NP Aux VP)
        grammer_s!(S -> Aux NP VP)
    
        grammer_s!(NP -> (Det) (Adj) N (PP))
        grammer_s!(NP -> "that" S)
        grammer_s!(NP -> Pronoun)
    
        grammer_s!(VP -> (Adv) V (NP) (PP) (Adv))
    
        grammer_s!(PP -> P NP)
    );
    let example = "The brown fox jumps over the lazy dog";
    
    let s = sentence_to_vec(example);
    let k = phrase::parse(&s, Part::S, &grammer);
    
    if let Ok((a, _)) = k {
        let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open("result.html").unwrap();
        a.to_html(&mut file);
    }
    else if let Err(e) = k {
        eprintln!("{}", e);
    }
}