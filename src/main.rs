use actix_web::{get, web, App, HttpResponse, http, HttpServer, Responder};
#[macro_use]
mod structure;
use structure::*;
mod phrase;
mod partoflang;
use partoflang::*;

// fn main() {
//     let grammer = grammer!(
//         grammer_s!(S -> NP VP)
//         grammer_s!(S -> NP Aux VP)
//         grammer_s!(S -> Aux NP VP)
//     
//         grammer_s!(NP -> (Det) (Adj) N (PP))
//         grammer_s!(NP -> "that" S)
//         grammer_s!(NP -> Pronoun)
//     
//         grammer_s!(VP -> (Adv) V (NP) (PP) (Adv))
//     
//         grammer_s!(PP -> P NP)
//     );
//     
//     let example = "I love you";
// 
//     let s = sentence_to_vec(&example);
//     let k = phrase::parse(&s, Part::S, &grammer);
//     
//     if let Ok((a, _)) = k {
//         let mut file = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("result.html").unwrap();
//         a.to_html(&mut file);
//     }
//     else if let Err(e) = k {
//         eprintln!("{}", e);
//     }
// }
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
        a.to_html(&mut s);
        HttpResponse::Ok().body(s.s)
    }
    else {
        let k = StatusCode::from_u16(502).unwrap();
        HttpResponse::build(k).body("Error!")
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}