use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use jumbf::parser::DataBox;

const SIMPLE_BOX: [u8; 38] = hex!(
    "00000026" // box size
    "6a756d64" // box type = 'jumd'
    "00000000000000000000000000000000" // UUID
    "03" // toggles
    "746573742e64657363626f7800" // label
);

pub fn simple_parse(c: &mut Criterion) {
    c.bench_function("simple data box", |b| {
        b.iter(|| DataBox::from_slice(&SIMPLE_BOX).unwrap());
    });
}

criterion_group!(benches, simple_parse);
criterion_main!(benches);
