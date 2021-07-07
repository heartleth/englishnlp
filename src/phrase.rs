use crate::partoflang::{ Word, DeterminedWord };
use crate::structure::*;
use std::result::Result;
use GrammerPart::*;

pub enum DiagramNodeEnum<'w> {
    Leaf(DeterminedWord<'w>),
    Node(DiagramLeaves<'w>),
    Template(&'w str)
}
use DiagramNodeEnum::*;
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
    // pub fn to_html(&self, stream :&mut dyn std::io::Write) {
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
            },
            Template(word) => {
                write!(stream, "<div style=\"padding:20px;border:solid 1px black\"><i>{}</i></div>", word).unwrap();
            }
        }
    }
}
type DiagramLeaves<'w> = Vec<DiagramNode<'w>>;
struct Candidate<'w> {
    structure: &'w UnPartedStructure<'w>,
    ready: DiagramLeaves<'w>,
    
    progress :usize,
    index :usize,
    
    alive :bool
}

impl<'w> Candidate<'w> {
    pub fn expects(&self)->Vec<usize> {
        let mut ret = Vec::new();
        for i in self.index .. self.structure.len() {
            let grammer_element = &self.structure[i];
            match grammer_element {
                Child(_) => {
                    ret.push(i);
                    break;
                },
                Voca(_) => {
                    ret.push(i);
                    break;
                },
                OptionalChild(_) => ret.push(i)
            }
        }
        return ret;
    }
    pub fn is_clear_low(&self)->bool {
        self.structure[self.index..self.structure.len()].iter().fold(true, |a, b|
           a && b.is_optional()
        )
    }
    pub fn is_clear(&self)->bool {
        self.index == self.structure.len()
    }
    pub fn prepare<'nw>(grammerset :&'nw Vec<UnPartedStructure<'nw>>)->Vec<Candidate<'nw>> {
        let mut candidates = Vec::new();
        for template in grammerset {
            candidates.push(Candidate {
                structure: &template,
                ready: Vec::new(),
                progress: 0,
                index: 0,
                alive: true
            });
        }
        return candidates;
    }
}

pub fn parse<'w>(s :&'w [Word<'w>], part :Part, grammer :&'w Grammer<'w>)->Result<(DiagramNode<'w>, usize), &'static str> {
    if let Some(grammerset) = grammer.part(part) {
        let mut candidates = Candidate::prepare(grammerset);
        loop {
            for candidate in &mut candidates {
                if !candidate.is_clear() {
                    let expects = if !candidate.is_clear_low() { candidate.expects() } else { vec![candidate.index] };
                    for i in expects {
                        let child = &candidate.structure[i];
                        let now = candidate.progress;
                        if let Child(part) = child {
                            if let Ok((child, fix)) = parse(&s[now..s.len()], *part, grammer) {
                                candidate.ready.push(child);
                                candidate.progress += fix;
                                candidate.index = i + 1;
                                break;
                            }
                            else {
                                candidate.alive = false;
                                break;
                            }
                        }
                        else if let OptionalChild(part) = child {
                            if let Ok((child, fix)) = parse(&s[now..s.len()], *part, grammer) {
                                candidate.ready.push(child);
                                candidate.progress += fix;
                                candidate.index = i + 1;
                                break;
                            }
                            if candidate.is_clear_low() {
                                candidate.index = i + 1;
                            }
                        }
                        else if let Voca(v) = child {
                            if &s[now].word == v {
                                candidate.ready.push(DiagramNode::new(
                                    Template(v), Part::Grammatic
                                ));
                                candidate.progress += 1;
                                candidate.index = i + 1;
                                break;
                            }
                            else {
                                candidate.alive = false;
                                break;
                            }
                        }
                    }
                }
            }
            candidates.retain(|x| x.alive);
            if candidates.len() == 1 {
                if candidates[0].is_clear() {
                    break;
                }
            }
            else if candidates.len() == 0 {
                return Err("No pattern matches.");
            }
        }
        
        let lastone = candidates.pop().unwrap();
        let progress = lastone.progress;
        Ok((DiagramNode::new(Node(lastone.ready), part), progress))
    }
    else {
        if let Some(first) = s.first() {
            if first.is_part(part) {
                Ok((DiagramNode::new(Leaf(s[0].determine(part)), part), 1))
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