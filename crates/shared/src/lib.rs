use serde::{Deserialize, Serialize};

pub type ResultV1<T> = anyhow::Result<T>;

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Comment {
    pub body: String,
    pub author: String,
}

pub enum Field {
    Int { name: String, description: String },
}

impl Field {
    pub fn int(name: &str, description: &str) -> Field {
        Field::Int {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub struct DatabaseObjectBuilder {
    table_name: String,
    fields: Vec<Field>,
}

impl DatabaseObjectBuilder {
    pub fn new(table_name: &str) -> Self {
        return Self {
            table_name: table_name.to_string(),
            fields: vec![],
        };
    }

    pub fn add(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }

    pub fn build(&self) {
        let create_query = format!("CREATE TABLE IF NOT EXISTS {}", self.table_name);

        let fields_string = self
            .fields
            .iter()
            .map(get_field_query_string)
            .collect::<Vec<_>>()
            .join(",");

        let create_query = format!("{create_query} ({fields_string});");

        dbg!(&create_query);
    }
}

fn get_field_query_string(field: &Field) -> String {
    match field {
        Field::Int { name, description } => {
            format!("BIGINT {}", name)
        }
    }
}
