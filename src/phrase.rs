use crate::partoflang::{ Word, DeterminedWord };
use std::collections::VecDeque;
use crate::structure::*;
use std::result::Result;
use GrammerPart::*;
mod psrtable;
mod display;

use psrtable::*;

#[derive(Debug, Clone)]
pub enum DiagramNodeEnum<'w> {
    Leaf(DeterminedWord<'w>),
    Node(DiagramLeaves<'w>)
}
use DiagramNodeEnum::*;
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ExpectmentParent {
    grammer :GrammerPart,
    parent :Coord
}
impl ExpectmentParent {
    pub fn from(up :&UnPartedStructure, parent :Coord)->Vec<ExpectmentParent> {
        up.iter().map(|e| ExpectmentParent {
            grammer: e.clone(),
            parent: parent
        }).collect()
    }
}
#[derive(Debug)]
pub struct Exepctment {
    top_level :usize,
    parent :Coord,
    then_remaining :Vec<ExpectmentParent>,
    part :Part,
    is_essential :bool,
    idx :usize
}
pub fn nexts<'g>(g :&'g Vec<ExpectmentParent>, level :usize)->Vec<Exepctment> {
    let mut ret = Vec::new();
    for i in 0..g.len() {
        let grammer_element = &g[i];
        match &grammer_element.grammer {
            Child(p) | OptionalChild(p) => {
                ret.push(Exepctment {
                    top_level: level,
                    then_remaining: g[i + 1..].to_vec(),
                    part: *p,
                    parent: grammer_element.parent,
                    is_essential: !grammer_element.grammer.is_optional() && i == 0,
                    idx: i
                });
                if !grammer_element.grammer.is_optional() {
                    break;
                }
            },
            Several(several) | OptionalSeveral(several) => {
                for candidate in several {
                    for expect in nexts(&ExpectmentParent::from(candidate, grammer_element.parent), level + 1) {
                        let remaining = [expect.then_remaining, g[i + 1..].to_vec()].concat();
                        ret.push(Exepctment {
                            top_level: level + 1,
                            parent: grammer_element.parent,
                            then_remaining: remaining,
                            part: expect.part,
                            is_essential: false,
                            idx: i + expect.idx
                        });
                    }
                }
                if !grammer_element.grammer.is_optional() {
                    break;
                }
            }
        }
    }
    ret
}

pub fn parse<'w>(s :&'w [Word<'w>], part :Part, grammer :&'w Grammer)->Result<DiagramNode<'w>, &'static str> {
    let mut table = Table::new();
    let mut q = VecDeque::new();
    let mut last = Vec::new();
    table.set(0, part, (0, Part::None, 0), (0, Part::None, 0), ExpectmentParent::from(grammer.part(part).unwrap(), (0, part, 0)), false);
    q.push_back((0, part, 0));

    while !q.is_empty() {
        let (pos, part, nth) = q.pop_front().unwrap();
        if pos < s.len() && table.has(pos, part, nth) {
            let front = table.get(pos, part, nth).unwrap();
            let expects = nexts(&front.structure, 0);
            if grammer.has(part) {
                for expect in expects {
                    let n = table.check(pos, expect.part);
                    table.set(pos, expect.part, (pos, part, nth), (pos, part, nth), [
                        ExpectmentParent::from(grammer.part(expect.part).unwrap_or(&Vec::new()), (pos, expect.part, n)),
                        expect.then_remaining
                    ].concat(), expect.is_essential);
                    q.push_back((pos, expect.part, n));
                }
            }
            else {
                if s[pos].is_part(part) {
                    let is_clear = front.structure.iter().fold(true, |a, b|
                        a && b.grammer.is_optional()
                    );
                    if pos == s.len() - 1 && !is_clear {
                        table.delete_family(pos, part, nth);
                    }
                    else if pos == s.len() - 1 {
                        last.push((pos, part, nth));
                    }
                    else {
                        for expect in expects {
                            let n = table.check(pos + 1, expect.part);
                            table.set(pos + 1, expect.part, (pos, part, nth), expect.parent, [
                                ExpectmentParent::from(grammer.part(expect.part).unwrap_or(&Vec::new()), (pos + 1, expect.part, n)),
                                expect.then_remaining
                            ].concat(), expect.is_essential);
                            q.push_back((pos + 1, expect.part, n));
                        }
                    }
                }
                else {
                    table.delete_family(pos, part, nth);
                }
            }
        }
        else {
            table.delete_family(pos, part, nth);
        }
    }
    for coord in last {
        let tree = table.tree(coord, s);
        if let Ok(tree) = tree {
            return Ok(tree);
        }
    }
    Err("")
}