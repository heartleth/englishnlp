use actix_web::{get, post, web, App, HttpResponse, http, HttpServer, Responder};
#[macro_use]
mod structure;
use structure::*;
mod phrase;
mod partoflang;
use phrase::*;

// use serde_derive::Deserialize;
// 
// struct WriteString {
//     ts :String,
//     pub s :String,
// }
// 
// impl std::io::Write for WriteString {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.s += std::str::from_utf8(buf).unwrap();
//         Ok(buf.len())
//     }
//     fn flush(&mut self) -> std::io::Result<()> {
//         self.s += &self.ts[..];
//         Ok(())
//     }
// }
// 
// #[derive(Deserialize)]
// struct DeString {
//     sentence: String
// }
// use http::StatusCode;
// 
// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().content_type("text/html").body("
//     <form method=\"post\" action=\"/literature\">
//         <textarea name=\"sentence\" row=20 col=30></textarea>
//         <input type=\"submit\" value=\"Submit\">
//     </form>
//     ")
// }
// 
// #[post("/literature")]
// async fn literature(form: web::Form<DeString>) -> HttpResponse {
//     let grammer = grammer!(
//         grammer_s!(S -> {{NP}|{SB}} (Aux) VP)
//         grammer_s!(NP -> {{Pronoun} | {(Det) (AP) N (PP) (SB)} | {SB}})
//         grammer_s!(VP -> (AdvP) V ({{(AP)}|{(NP) ({{NP}|{PP}|{SB}})}}) (XPP))
//         grammer_s!(AP -> (AdvP) Adj (AP) ({{PP}|{SB}}))
//         grammer_s!(XP -> {{Adv}|{PP}})
//         grammer_s!(AdvP -> Adv (AdvP))
//         grammer_s!(XPP -> XP (XPP))
//         grammer_s!(PP -> P (NP))
//         grammer_s!(Det -> {{Art}|{Dem}})
//         grammer_s!(SB -> Comp S)
//         grammer_s!(Aux -> ({{Inf}|{Modal}}) (Perf) (Prog))
//     );
//     let s = &form.sentence;
//     let mut stream = WriteString {
//         ts :String::new(),
//         s :String::new()
//     };
//     for sentence in s.split('.') {
//         let s = partoflang::sentence_to_vec(sentence);
//         let k = parse(&s, Part::NP, &grammer);
//         if let Ok((a, _)) = k {
//             a.to_html(&mut stream);
//         }
//         else {
//             let k = StatusCode::from_u16(502).unwrap();
//             return HttpResponse::build(k).body("Error!");
//         }
//     }
//     HttpResponse::Ok().body(stream.s)
// }
// 
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//             .service(literature)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

fn main() {
    let grammer = grammer!(
        grammer_s!(S -> {{NP}|{SB}} (Aux) VP)
        grammer_s!(NP -> {{Pronoun} | {(Det) (AP) N (PP) (SB)} | {SB}})
        grammer_s!(AP -> (AdvP) Adj (AP) ({{PP}|{SB}}))
        grammer_s!(VP -> (AdvP) V ({{(AP)}|{(NP) ({{NP}|{PP}|{SB}})}}) (XPP))
        grammer_s!(XP -> {{Adv}|{PP}})
        grammer_s!(AdvP -> Adv (AdvP))
        grammer_s!(XPP -> XP (XPP))
        grammer_s!(PP -> P (NP))
        grammer_s!(Det -> {{Art}|{Dem}})
        grammer_s!(SB -> Comp S)
        grammer_s!(Aux -> ({{Inf}|{Modal}}) (Perf) (Prog))
    );
    let s = "when lenin made ussr";
    
    let p = partoflang::sentence_to_vec(s);
    let k = parse(&p[..], Part::NP, &grammer);
    println!("{:?}", k);
}

#[cfg(test)]
mod tests {
    use crate::*;
    fn test_sentence(s :&str, f :&mut dyn std::io::Write) {
        let grammer = grammer!(
            grammer_s!(S -> {{NP}|{SB}} (Aux) VP)
            grammer_s!(NP -> {{Pronoun} | {(Det) (AP) N (PP) (SB)} | {SB}})
            grammer_s!(AP -> (AdvP) Adj (AP) ({{PP}|{SB}}))
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
        let p = parse(&s, Part::NP, &grammer).unwrap();
        let pr = p.1;
        let mut f = std::fs::OpenOptions::new().append(true).write(true).open("res.html").unwrap();
        p.0.to_html(&mut f);
        assert_eq!(pr, s.len());
    }
    #[test]
    fn tst() {
        let mut st = std::fs::OpenOptions::new().create(true).truncate(true).write(true).open("res.html").unwrap();

        test_sentence("fifth american president", &mut st);
        test_sentence("major moments in the history of writing", &mut st);
        test_sentence("marxism", &mut st);
        test_sentence("english words from chinese words", &mut st);
        test_sentence("old chinese phonology", &mut st);
        test_sentence("that lenin made USSR", &mut st);
    }
}