use crate::partoflang::DeterminedWord;
use crate::structure::*;

#[derive(Debug, Clone)]
pub enum DiagramNodeEnum<'w> {
    Leaf(DeterminedWord<'w>),
    Node(DiagramLeaves<'w>)
}
use DiagramNodeEnum::*;
#[derive(Debug, Clone)]
pub struct DiagramNode<'w> {
    pub node: DiagramNodeEnum<'w>,
    pub part: Part
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

    pub fn score(&self)->usize {
        match &self.node {
            Leaf(w) => w.part as usize,
            Node(l) => l.iter().fold(0, |a, b| a + b.score())
        }
    }
}

type DiagramLeaves<'w> = Vec<DiagramNode<'w>>;