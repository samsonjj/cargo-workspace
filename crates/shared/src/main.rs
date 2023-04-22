use shared::{DatabaseObjectBuilder, Field};

#[tokio::main]
async fn main() {
    // let s =
    let dbbuilder = DatabaseObjectBuilder::new("foo_table")
        .add(Field::int("hello", "description test"))
        .add(Field::int("goodbye", "description test"))
        .build();
}
