use std::collections::HashMap;
use crate::partoflang::Parts;

/// ## Proposed english grammer
/// ### References
/// *Lexical Relations and Grammatical Relations* (http://www.people.fas.harvard.edu/~ctjhuang/lecture_notes/lecch5.html)
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Part {
    Pronoun,
    S, // {NP|S'} (Aux) VP
    NP, // {Pronoun|(Det) (AP) N (PP) (S')}
    VP, // (AdvP) V {(AP)|(NP) ({NP | PP | S'})} (XP*)
    AP, // (deg) A ({PP|S'})
    PP, // P (NP)
    Det, // Determiner, {Art|Dem|NP_Poss}
    X, // X ((Conj) X...) Conj X
    SB, // = S', (Comp) S
    Aux, // ({Inf|Modal}) (Perf) (Prog)
    AdvP,
    /// The category XP is intended to represent several of the categories that an adverbial expression may take
    XP,
    XPP,

    N, // -----> books, ideas, mother, man, student, girl, house, friend, cement, pilot, . . . .
    V, // -----> kick, laugh, cry, buy, live, tell, give, put, say, . . . .
    Adj, // -----> good, bad, colorless, green, long, redundant, . . . .
    P, // -----> at, in, under, on, through, up, . . . .
    Art, // ----> a, the, some, . . . .
    Dem, // ----> this, that, these, those
    Deg, // ----> very, extremely, . . . .
    Conj, // ----> and, but, or, . . . .
    Comp, // ----> that, if, whether, for, why, who, etc.
    Inf, // ----> to
    Modal, // ----> can, may, must, will, shall, could, might, ....
    Perf, // ----> have
    Prog, // ----> be
    Adv // ----> quickly, suddenly, carefully, etc.
}

impl Part {
    pub fn from_string(s :&str) -> Part {
        match s {
            "preposition"
            | "P" => Part::P,
            "Det" => Part::Det,
            "definite article"
            | "indefinite article"
            | "Art" => Part::Art,
            "adjective"
            | "Adj" => Part::Adj,
            "auxiliary verb" => Part::Modal,
            "Aux" => Part::Aux,
            "Deg" => Part::Deg,
            "adverb"
            | "Adv" => Part::Adv,
            "noun"
            | "N" => Part::N,
            "verb"
            | "V" => Part::V,
            "pronoun"
            | "Pronoun" => Part::Pronoun,
            "NP" => Part::NP,
            "VP" => Part::VP,
            "PP" => Part::PP,
            "S" => Part::S,
            "conjunction"
            | "Conj" => Part::Conj,
            &_ => Part::N
        }
    }
    pub fn to_multi(self)->Parts {
        let mut ret = Parts::new();
        ret.insert(self);
        ret
    }
}

#[derive(Debug, Clone)]
pub enum GrammerPart {
    Child(Part),
    OptionalChild(Part),
    Several(Vec<UnPartedStructure>),
    OptionalSeveral(Vec<UnPartedStructure>)
}

impl GrammerPart {
    pub fn is_optional(&self)->bool {
        match self {
            GrammerPart::OptionalChild(_) => true,
            GrammerPart::OptionalSeveral(_) => true,
            _ => false
        }
    }
}

pub type UnPartedStructure = Vec<GrammerPart>;
pub type Structure = (Part, UnPartedStructure);
pub struct Grammer {
    hash: HashMap<Part, UnPartedStructure>
}

impl Grammer {
    pub fn push(mut self, item: Structure)->Grammer {
        self.hash.insert(item.0, item.1);
        self
    }
    pub fn part(&self, name :Part)->Option<&UnPartedStructure> {
        self.hash.get(&name)
    }
    pub fn new()->Grammer {
        Grammer {
            hash: HashMap::new()
        }
    }
}

#[macro_export]
macro_rules! grammer_s_meta {
    ($c:ident) => {
        GrammerPart::Child(Part::$c)
    };
    (($c:ident)) => {
        GrammerPart::OptionalChild(Part::$c)
    };

    // { NP }
    // { SB } => {{ NP } | { SB }}
    ({{$( $c1:tt )*} $(| {$( $c:tt )*})*}) => {
        GrammerPart::Several(vec![
            grammer_s!($( $c1 )*)
            $(, grammer_s!($( $c )*))*
        ])
    };
    (({{$( $c1:tt )*} $(| {$( $c:tt )*})*})) => {
        GrammerPart::OptionalSeveral(vec![
            grammer_s!($( $c1 )*)
            $(, grammer_s!($( $c )*))*
        ])
    };
}

#[macro_export]
macro_rules! grammer_s {
    () => {};
    ($f:ident -> $c1:tt $( $c:tt )*) => {
        (Part::$f, vec![grammer_s_meta!($c1) $(,grammer_s_meta!($c))*])
    };
    ($c1:tt $( $c:tt )*) => {
        vec![grammer_s_meta!($c1) $(,grammer_s_meta!($c))*]
    };
}

#[macro_export]
macro_rules! grammer {
    ($($e:expr)*) => {
        Grammer::new()$(.push($e))*
    };
}