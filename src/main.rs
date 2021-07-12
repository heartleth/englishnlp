use actix_web::{get, post, web, App, HttpResponse, http, HttpServer, Responder};
#[macro_use]
mod structure;
use structure::*;
mod phrase;
mod partoflang;
use phrase::*;

use serde_derive::Deserialize;

struct WriteString {
    ts :String,
    pub s :String,
}

impl std::io::Write for WriteString {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.s += std::str::from_utf8(buf).unwrap();
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.s += &self.ts[..];
        Ok(())
    }
}

#[derive(Deserialize)]
struct DeString {
    sentence: String
}
use http::StatusCode;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body("
    <form method=\"post\" action=\"/literature\">
        <textarea name=\"sentence\" row=20 col=30></textarea>
        <input type=\"submit\" value=\"Submit\">
    </form>
    ")
}

#[post("/literature")]
async fn literature(form: web::Form<DeString>) -> HttpResponse {
    let grammer = grammer!(
        grammer_s!(S -> {{NP}|{SB}} (Aux) VP)
        grammer_s!(NP -> {{Pronoun} | {(Det) (AP) N (PP) (SB)} | {SB}})
        grammer_s!(VP -> (AdvP) V ({{(AP)}|{(NP) ({{NP}|{PP}|{SB}})}}) (XPP))
        grammer_s!(AP -> (AdvP) Adj (AP) ({{PP}|{SB}}))
        grammer_s!(XP -> {{Adv}|{PP}})
        grammer_s!(AdvP -> Adv (AdvP))
        grammer_s!(XPP -> XP (XPP))
        grammer_s!(PP -> P (NP))
        grammer_s!(Det -> {{Art}|{Dem}})
        grammer_s!(SB -> Comp S)
        grammer_s!(Aux -> ({{Inf}|{Modal}}) (Perf) (Prog))
    );
    let s = &form.sentence;
    let mut stream = WriteString {
        ts :String::new(),
        s :String::new()
    };
    for sentence in s.split('.') {
        let s = partoflang::sentence_to_vec(sentence);
        let k = parse(&s, Part::VP, &grammer);
        if let Ok((a, _)) = k {
            a.to_html(&mut stream);
        }
        else {
            let k = StatusCode::from_u16(502).unwrap();
            return HttpResponse::build(k).body("Error!");
        }
    }
    HttpResponse::Ok().body(stream.s)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(literature)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use crate::*;
    fn test_sentence(s :&str) {
        let grammer = grammer!(
            grammer_s!(S -> {{NP}|{SB}} (Aux) VP)
            grammer_s!(NP -> {{Pronoun} | {(Det) (AP) N (PP) (SB)} | {SB}})
            grammer_s!(VP -> (AdvP) V ({{AP}|{(NP) ({{NP}|{PP}|{SB}})}}) (XPP))
            grammer_s!(AP -> (AdvP) Adj (AP) ({{PP}|{SB}}))
            grammer_s!(XP -> {{Adv}|{PP}})
            grammer_s!(AdvP -> Adv (AdvP))
            grammer_s!(XPP -> XP (XPP))
            grammer_s!(PP -> P (NP))
            grammer_s!(Det -> {{Art}|{Dem}})
            grammer_s!(SB -> Comp S)
            grammer_s!(Aux -> ({{Inf}|{Modal}}) (Perf) (Prog))
        );
        let s = partoflang::sentence_to_vec(s);
        let p = parse(&s, Part::S, &grammer).unwrap();
        let mut st = std::fs::OpenOptions::new().append(true).write(true).open("res.html").unwrap();
        let pr = p.1;
        p.0.to_html(&mut st);
        assert_eq!(pr, s.len());
        // .and_then(|e|Ok(e.1))
    }
    #[test]
    fn tst() {
        test_sentence("The men would put the book");
        test_sentence("John explained Bill the theory");
        test_sentence("The man elapsed");
        test_sentence("The man from Ohio met");
        test_sentence("He jumped off");
        test_sentence("The thief broke in");
        test_sentence("You should look beyond");
        test_sentence("He destroyed my plan");
        test_sentence("They discussed the issue");
        test_sentence("Barry studies music");
        test_sentence("Josephine teaches English");
        test_sentence("John handed a toy to the baby");
    }
}