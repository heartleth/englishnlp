use criterion::{ criterion_group, criterion_main, Criterion };
extern crate englishnlp;
use englishnlp::tests::test_sentence;

fn test() {
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
    test_sentence("John talked to Bill in the garden");
    test_sentence("John put the plants in the garden");
    test_sentence("I don't believe the claim that he won the prize");
    test_sentence("Bill was praised by the President");
    test_sentence("My brother suddenly appeared at the doorstep");
    test_sentence("I have been helping");
    test_sentence("He will retire in style in a year");
    test_sentence("The girl and her father said that they have been discussing the possibilities");
    test_sentence("A toy was handed to the baby by John");
    test_sentence("The baby was handed a toy by John");
    test_sentence("The boy kicked the ball");
    test_sentence("The boy was scolded by his sister");
    test_sentence("The boy was given a toy");
    test_sentence("The boy enjoyed the game");
    test_sentence("He looks stupid");
    test_sentence("Mary felt very sad");
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