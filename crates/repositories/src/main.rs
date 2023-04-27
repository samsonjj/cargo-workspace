use repositories::builder::{DatabaseObjectBuilder, Field};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = repositories::get_ready_client().await?;

    // let create_query = DatabaseObjectBuilder::new("foo_table")
    //     .add(Field::int("hello", "description test"))
    //     .add(Field::int("goodbye", "description test"))
    //     .build();

    // client.execute(&create_query[..], &[]).await?;

    Ok(())
}

struct PageView {
    name: String,
}

// impl DbObject for PageView {
//     fn get_fields(builder: &mut DatabaseObjectBuilder) {
//         builder.add(Field::int("Hello", "description test"));
//     }
// }
