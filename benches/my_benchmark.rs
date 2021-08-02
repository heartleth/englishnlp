use criterion::{ criterion_group, criterion_main, Criterion };
extern crate englishnlp;
use englishnlp::tests::test_sentence;

fn test() {
    let mut file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open("res.html").unwrap();
    englishnlp::phrase::DiagramNode::highlight_css(&mut file);

    test_sentence("The men would put the book", &mut file);
    test_sentence("John explained Bill the theory", &mut file);
    test_sentence("The man elapsed", &mut file);
    test_sentence("The man from Ohio met", &mut file);
    test_sentence("He jumped off", &mut file);
    test_sentence("The thief broke in", &mut file);
    test_sentence("You should look beyond", &mut file);
    test_sentence("He destroyed my plan", &mut file);
    test_sentence("They discussed the issue", &mut file);
    test_sentence("Barry studies music", &mut file);
    test_sentence("Josephine teaches English", &mut file);
    test_sentence("John handed a toy to the baby", &mut file);
    test_sentence("John talked to Bill in the garden", &mut file);
    test_sentence("John put the plants in the garden", &mut file);
    test_sentence("I don't believe the claim that he won the prize", &mut file);
    test_sentence("Bill was praised by the President", &mut file);
    test_sentence("My brother suddenly appeared at the doorstep", &mut file);
    test_sentence("I have been helping", &mut file);
    test_sentence("He will retire in style in a year", &mut file);
    test_sentence("The girl and her father said that they have been discussing the possibilities", &mut file);
    test_sentence("A toy was handed to the baby by John", &mut file);
    test_sentence("The baby was handed a toy by John", &mut file);
    test_sentence("The boy kicked the ball", &mut file);
    test_sentence("The boy was scolded by his sister", &mut file);
    test_sentence("The boy was given a toy", &mut file);
    test_sentence("The boy enjoyed the game", &mut file);
    test_sentence("He looks stupid", &mut file);
    test_sentence("Mary felt very sad", &mut file);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| b.iter(|| test()));
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);