use crate::*;

pub fn test_sentence(s :&str) {
    let grammer = grammer!(
        grammer_s!(S -> {{NP}|{SB}} (Aux) VP)
        grammer_s!(NP -> {{Pronoun} | {(Det) (AP) N (PP) (SB)} | {SB}})
        grammer_s!(AP -> ({{AdvP}|{Deg}}) Adj (AP) ({{PP}|{SB}}))
        grammer_s!(VP -> (AdvP) V ({{(AP)}|{(NP) ({{NP}|{PP}|{SB}})}}) (XPP))
        grammer_s!(XP -> {{Adv}|{PP}})
        grammer_s!(AdvP -> Adv (AdvP))
        grammer_s!(XPP -> XP (XPP))
        grammer_s!(PP -> P (NP))
        grammer_s!(Det -> {{Art}|{Dem}})
        grammer_s!(SB -> Comp S)
        grammer_s!(Aux -> ({{Inf}|{Modal}}) (Perf) (Prog))
    );
    let s = partoflang::sentence_to_vec(s);
    parse(&s, Part::S, &grammer).unwrap();
}