use derive_ent::Ent;

struct Thing;

#[derive(Ent)]
struct Foo {
    bar: Thing,
}

fn main() {}
