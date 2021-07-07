use std::collections::HashMap;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub enum Part {
    N, NP,
    V, VP,
    Det,
    S,
    Pronoun,
    P, PP, Adv,
    Aux,
    Adj,
    Grammatic
}

impl Part {
    pub fn from_string(s :&str) -> Part {
        match s {
            "preposition" | "P"        => Part::P,
            "definite article" | "Det" => Part::Det,
            "adjective" | "Adj"        => Part::Adj,
            "auxiliary verb" | "Aux"   => Part::Aux,
            "adverb" | "Adv"           => Part::Adv,
            "noun" | "N"               => Part::N,
            "verb" | "V"               => Part::V,
            "pronoun"                  => Part::Pronoun,
            "NP"                       => Part::NP,
            "VP"                       => Part::VP,
            "PP"                       => Part::PP,
            "S"                        => Part::S,
            &_                         => Part::N
        }
    }
}

#[derive(Debug)]
pub enum GrammerPart<'v> {
    Voca(&'v str),
    // OptionalVoca(&'v str),
    Child(Part),
    OptionalChild(Part)
}

impl<'v> GrammerPart<'v> {
    pub fn is_optional(&self)->bool {
        match self {
            GrammerPart::OptionalChild(_) => true,
            _ => false
        }
    }
}

pub type UnPartedStructure<'v> = Vec<GrammerPart<'v>>;
pub type Structure<'v> = (Part, UnPartedStructure<'v>);
pub struct Grammer<'v> {
    hash: HashMap<Part, Vec<UnPartedStructure<'v>>>
}

impl<'v> Grammer<'v> {
    pub fn push(mut self, item: Structure<'v>)->Grammer<'v> {
        if !self.hash.contains_key(&item.0) {
            self.hash.insert(item.0, vec![item.1]);
        }
        else {
            self.hash.get_mut(&item.0).unwrap().push(item.1);
        }
        self
    }
    pub fn part<'s>(&self, name :Part)->Option<&Vec<UnPartedStructure<'v>>> {
        self.hash.get(&name)
    }
    pub fn new()->Grammer<'v> {
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
    ($c:expr) => {
        GrammerPart::Voca(stringify!($c))
    };
    // (($c:expr)) => {
    //     GrammerPart::OptionalVoca(stringify!($c))
    // };
}

#[macro_export]
macro_rules! grammer_s {
    () => {};
    ($f:ident -> $c1:tt $( $c:tt )*) => {
        (Part::$f, vec![grammer_s_meta!($c1) $(,grammer_s_meta!($c))*])
    };
}

#[macro_export]
macro_rules! grammer {
    ($($e:expr)*) => {
        Grammer::new()$(.push($e))*
    };
}