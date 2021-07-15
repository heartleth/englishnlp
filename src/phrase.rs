use crate::partoflang::{ Word, DeterminedWord };
use crate::structure::*;
use std::result::Result;
use GrammerPart::*;
mod display;

#[derive(Debug)]
pub enum DiagramNodeEnum<'w> {
    Leaf(DeterminedWord<'w>),
    Node(DiagramLeaves<'w>)
}
use DiagramNodeEnum::*;
#[derive(Debug)]
pub struct DiagramNode<'w> {
    node: DiagramNodeEnum<'w>,
    part: Part
}
impl<'w> DiagramNode<'w> {
    pub fn new<'nw>(e :DiagramNodeEnum<'nw>, part :Part)->DiagramNode<'nw> {
        DiagramNode {
            node: e,
            part: part
        }
    }

    pub fn to_html(&self, stream :&mut dyn std::io::Write) {
        match &self.node {
            Leaf(word) => {
                write!(stream, "<div style=\"padding:20px;border:solid 1px black\"><strong>{:?}</strong><p>{}</p></div>", word.part, word.word).unwrap();
            },
            Node(node) => {
                write!(stream, "<table style=\"border:solid 1px black\"><tr><th colspan=\"{}\">{:?}</th></tr><tr>", node.len(), self.part).unwrap();
                for child in node {
                    write!(stream, "<td style=\"vertical-align:top;\">").unwrap();
                    child.to_html(stream);
                    write!(stream, "</td>").unwrap();
                }
                write!(stream, "</tr></table>").unwrap();
            }
        }
    }
}

type DiagramLeaves<'w> = Vec<DiagramNode<'w>>;
struct Candidate<'w> {
    structure: UnPartedStructure,
    ready: DiagramLeaves<'w>,
    
    progress :usize,
    level :usize,
    
    alive :bool
}
struct NextCandidate<'w> {
    structure :UnPartedStructure,
    ready :DiagramNode<'w>,
    progress :usize,
    part :Part
}
#[derive(Debug)]
pub struct ExepctMent<'g> {
    top :&'g UnPartedStructure,
    top_level :usize,
    then_remaining :Vec<GrammerPart>,
    part :Part,
    is_several :bool,
    idx :usize
}
pub fn nexts<'g>(g :&'g UnPartedStructure, level :usize)->Vec<ExepctMent<'g>> {
    let mut ret = Vec::new();
    for i in 0..g.len() {
        let grammer_element = &g[i];
        match grammer_element {
            Child(p) | OptionalChild(p) => {
                ret.push(ExepctMent {
                    top: g,
                    top_level: level,
                    then_remaining: g[i + 1..].to_vec(),
                    part: *p,
                    is_several: grammer_element.is_optional(),
                    idx: i
                });
                if !grammer_element.is_optional() {
                    break;
                }
            },
            Several(several) | OptionalSeveral(several) => {
                for candidate in several {
                    for expect in nexts(candidate, level + 1) {
                        let remaining = [expect.then_remaining, g[i + 1..].to_vec()].concat();
                        ret.push(ExepctMent {
                            top: g,
                            top_level: level + 1,
                            then_remaining: remaining,
                            part: expect.part,
                            is_several: true,
                            idx: i + expect.idx
                        });
                    }
                }
                if !grammer_element.is_optional() {
                    break;
                }
            }
        }
    }
    ret
}
impl<'w> Candidate<'w> {
    pub fn is_clear_low(&self)->bool {
        self.structure.iter().fold(true, |a, b|
           a && b.is_optional()
        )
    }
    pub fn is_clear(&self)->bool {
        self.structure.len() == 0
    }
    pub fn prepare<'nw>(grammerset :&'nw UnPartedStructure)->Candidate<'nw> {
        Candidate {
            structure: grammerset.clone(),
            ready: Vec::new(),
            progress: 0,
            level: 0,
            alive: true
        }
    }
}

pub fn parse<'w>(s :&'w [Word<'w>], part :Part, grammer :&'w Grammer)->Result<(DiagramNode<'w>, usize), &'static str> {
    if s.len() == 0 {
        return Err("Phrase is empty.");
    }
    else if let Some(grammerset) = grammer.part(part) {
        let mut candidate = Candidate::prepare(grammerset);
        loop {
            if !candidate.is_clear() {
                let mut ok = false;
                let expects = nexts(&candidate.structure, candidate.level);
                let mut new_candidates = Vec::new();
                let now = candidate.progress;
                for i in expects {
                    if let Ok((child, fix)) = parse(&s[now..], i.part, grammer) {
                        new_candidates.push(NextCandidate {
                            progress: fix,
                            part: child.part,
                            ready: child,
                            structure: i.then_remaining
                        });
                        ok = true;
                    }
                }
                if ok {
                    new_candidates.retain(|x|!x.structure.iter().any(|x|!x.is_optional())||x.progress+now<s.len());
                    new_candidates.sort_by(|a, b|{
                        let r = a.progress.cmp(&b.progress);
                        if r == std::cmp::Ordering::Equal {
                            let k = a.structure.len().cmp(&b.structure.len());
                            match k {
                                std::cmp::Ordering::Equal => if a.part == b.part { std::cmp::Ordering::Equal }
                                else if a.part > b.part { std::cmp::Ordering::Less }
                                else { std::cmp::Ordering::Greater },
                                _ => k
                            }
                        }
                        else { r }
                    });
                    if new_candidates.is_empty() {
                        return Err("Not my position.");
                    }
                    let lastone :NextCandidate = new_candidates.pop().unwrap();
                    candidate.progress += lastone.progress;
                    candidate.structure = lastone.structure;
                    candidate.ready.push(lastone.ready);
                }
                else {
                    if !candidate.is_clear_low() {
                        candidate.alive = false;
                    }
                    else {
                        candidate.structure = Vec::new();
                    }
                }
            }
            if !candidate.alive {
                return Err("No pattern matches.");
            }
            if candidate.is_clear() {
                break;
            }
        }
        let lastone = candidate;
        let progress = lastone.progress;
        if progress == 0 {
            return Err("Phrase is empty.");
        }
        else {
            Ok((DiagramNode::new(Node(lastone.ready), part), progress))
        }
    }
    else {
        if let Some(first) = s.first() {
            if first.is_part(part) {
                Ok((DiagramNode::new(Leaf(s[0].determine(part).unwrap()), part), 1))
            }
            else {
                Err("part of speech doesn't match.")
            }
        }
        else {
            Err("part of speech doesn't match.")
        }
    }
}