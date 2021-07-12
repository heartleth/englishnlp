use super::*;

impl<'w> DiagramNode<'w> {
    // pub fn highlight_(&self, depth :usize)->String {
    //     match &self.node {
    //         Leaf(word) => {
    //             if word.part == Part::V && depth == 1 {
    //                 format!("<div class=\"i\"><ruby><rb>{}</rb><rp>(</rp><rt>V</rt><rp>)</rp></ruby><span class=\"slash\">/</span></div>", word.word)
    //             }
    //             else {
    //                 format!("<div class=\"i\">{}</div>", word.word)
    //             }
    //         },
    //         Node(node) => {
    //             let mut ret = String::new();
    //             if self.part == Part::S {
    //                 if depth > 0 {
    //                     ret += "<span class=slash>[</span>";
    //                 }
    //                 for child in node {
    //                     if child.part == Part::NP {
    //                         if depth == 0 {
    //                             ret += &format!("<ruby><rb>{}</rb><rp>(</rp><rt>S</rt><rp>)</rp></ruby>", child.highlight_(depth + 1))[..];
    //                         }
    //                         else {
    //                             ret += &child.highlight_(depth + 1)[..];
    //                         }
    //                         ret += "<span class=\"slash\">/</span>";
    //                     }
    //                     else if child.part == Part::VP {
    //                         ret += &format!("{}", child.highlight_(depth + 1))[..];
    //                     }
    //                     else if child.part == Part::PP {
    //                         ret += &format!("({})", child.highlight_(depth + 1))[..];
    //                     }
    //                     else {
    //                         ret += &child.highlight_(depth + 1)[..];
    //                     }
    //                 }
    //                 if depth > 0 {
    //                     ret += "<span class=slash>]</span>";
    //                 }
    //             }
    //             else if self.part == Part::VP {
    //                 for child in node {
    //                     if child.part == Part::NP {
    //                         ret += &format!("<div class=\"i\"><ruby><rb>{}</rb><rp>(</rp><rt>O</rt><rp>)</rp></ruby></div>", child.highlight_(depth))[..];
    //                     }
    //                     else {
    //                         ret += &child.highlight_(depth)[..];
    //                     }
    //                 }
    //             }
    //             else {
    //                 for child in node {
    //                     ret += &child.highlight_(depth + 1)[..];
    //                 }
    //             }
    //             ret
    //         }
    //     }
    // }
    // pub fn highlight(&self, stream :&mut dyn std::io::Write) {
    //     write!(stream, "<style>#highlightbox .d{{
    //         display:inline;
    //         background-image:url(https://media.discordapp.net/attachments/849082780863168532/862924869184651294/triangle-1.png?width=720&height=625);
    //         background-size:100% 100%;
    //     }}
    //     #highlightbox .slash{{color:red;font-size:40px;}}
    //     #highlightbox .i{{display:inline;margin:5px;font-size:30px;}}
    //     #highlightbox rt{{color:red;font-size:15px;}}
    //     #highlightbox {{margin:8px;display:inline;}}
    //     </style><div id=highlightbox>{}</div>", self.highlight_(0)).unwrap();
    // }
}