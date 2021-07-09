use actix_web::{get, post, web, App, HttpResponse, http, HttpServer, Responder};
#[macro_use]
mod structure;
use structure::*;
mod phrase;
mod partoflang;
use partoflang::*;

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

#[get("/single")]
async fn hello(req :web::Query<DeString>) -> impl Responder {
    let grammer = grammer!(
        grammer_s!(S -> NP VP)
        grammer_s!(S -> NP Aux VP)
        grammer_s!(S -> Aux NP VP)

        grammer_s!(NP -> (Det) (Adj) N (PP))
        grammer_s!(NP -> "that" S)
        grammer_s!(NP -> Pronoun)

        grammer_s!(VP -> (Adv) V (NP) (PP) (Adv))

        grammer_s!(PP -> P NP)
    );

    let sentence = &req.sentence;

    let s = sentence_to_vec(sentence);
    let k = phrase::parse(&s, Part::S, &grammer);
    
    if let Ok((a, _)) = k {
        let mut s = WriteString {
            ts :String::new(),
            s :String::new()
        };
        a.highlight(&mut s);
        HttpResponse::Ok().body(s.s)
    }
    else {
        let k = StatusCode::from_u16(502).unwrap();
        HttpResponse::build(k).body("Error!")
    }
}

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
        grammer_s!(S -> NP VP)
        grammer_s!(S -> NP Aux VP)
        grammer_s!(S -> Aux NP VP)

        grammer_s!(NP -> (Det) (Adj) N (PP))
        grammer_s!(NP -> "that" S)
        grammer_s!(NP -> Pronoun)

        grammer_s!(VP -> (Adv) V (NP) (PP) (Adv))

        grammer_s!(PP -> P NP)
    );
    let s = &form.sentence;
    let mut stream = WriteString {
        ts :String::new(),
        s :String::new()
    };
    for sentence in s.split('.') {
        let sentence = sentence.trim();
        let s = sentence_to_vec(sentence);
        let k = phrase::parse(&s, Part::S, &grammer);
        if let Ok((a, _)) = k {
            a.highlight(&mut stream);
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
            .service(hello)
            .service(index)
            .service(literature)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}