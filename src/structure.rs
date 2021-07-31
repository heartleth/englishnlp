use std::collections::HashMap;
use crate::partoflang::Parts;

/// ## Proposed english grammer
/// ### References
/// *Lexical Relations and Grammatical Relations* (http://www.people.fas.harvard.edu/~ctjhuang/lecture_notes/lecch5.html)
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy, PartialOrd)]
pub enum Part {
    Pronoun = 1,
    S = 29,       // {NP|S'} (Aux) VP
    NP = 39,      // {Pronoun|(Det) (AP) N (PP) (S')}
    VP = 49,      // (AdvP) V {(AP)|(NP) ({NP | PP | S'})} (XP*)
    AP = 59,      // (deg) A ({PP|S'})
    PP = 69,      // P (NP)
    Det = 4,     // Determiner, {Art|Dem|NP_Poss}
    // X = 89,       // X ((Conj) X...) Conj X
    SB = 99,      // = S', (Comp) S
    Aux = 109,     // ({Inf|Modal}) (Perf) (Prog)
    AdvP = 119,
    /// The category XP is intended to represent several of the categories that an adverbial expression may take
    XP = 129,
    XPP = 139,

    N = 15, // -----> books, ideas, mother, man, student, girl, house, friend, cement, pilot, . . . .
    V = 13, // -----> kick, laugh, cry, buy, live, tell, give, put, say, . . . .
    Adj = 14, // -----> good, bad, colorless, green, long, redundant, . . . .
    P = 12, // -----> at, in, under, on, through, up, . . . .
    Art = 3, // ----> a, the, some, . . . .
    Dem = 5, // ----> this, that, these, those
    Deg = 7, // ----> very, extremely, . . . .
    Conj = 6, // ----> and, but, or, . . . .
    Comp = 2, // ----> that, if, whether, for, why, who, etc.
    Inf = 8, // ----> to
    Modal = 11, // ----> can, may, must, will, shall, could, might, ....
    Perf = 10, // ----> have
    Prog = 9, // ----> be
    Adv = 16, // ----> quickly, suddenly, carefully, etc.
    None = 999
}

impl Part {
    pub fn from_string(s :&str) -> Part {
        match s {
            "preposition"
            | "P" => Part::P,
            "Det" => Part::Det,
            "Prog" => Part::Prog,
            "definite article"
            | "indefinite article"
            | "Art" => Part::Art,
            "adjective"
            | "Adj" => Part::Adj,
            "auxiliary verb"
            | "Modal" => Part::Modal,
            "Aux" => Part::Aux,
            "Deg" => Part::Deg,
            "Dem" => Part::Dem,
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
            "Comp" => Part::Comp,
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
    fn to_string(&self)->String {
        match self {
            GrammerPart::Child(c) => format!("{:?}", c),
            GrammerPart::OptionalChild(c) => format!("({:?})", c),
            GrammerPart::Several(ss) => {
                format!("{{{}}}", ss.iter().map(|s| s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join(" | "))
            },
            GrammerPart::OptionalSeveral(ss) => {
                format!("({{{}}})", ss.iter().map(|s| s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).collect::<Vec<String>>().join(" | "))
            },
        }
    }
}
use std::fmt;
impl fmt::Display for GrammerPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
    pub fn has(&self, name :Part)->bool {
        self.hash.contains_key(&name)
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