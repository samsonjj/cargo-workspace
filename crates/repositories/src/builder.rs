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

struct Ent<T> {
    inner: T,
    q_create: String,
}

pub trait EntConfig {
    fn config(builder: &mut DatabaseObjectBuilder) -> Vec<Field>;

    fn init(&mut self) {
        let mut builder = DatabaseObjectBuilder::new();
        Self::config(&mut builder);
        let create_string = builder.build();
    }
}

impl DatabaseObjectBuilder {
    pub fn new() -> Self {
        return Self {
            table_name: "".to_string(),
            fields: vec![],
        };
    }

    pub fn table_name(&mut self, table_name: &str) {
        self.table_name = table_name.to_string();
    }

    pub fn field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }

    pub fn build(&self) -> String {
        let create_query = format!("CREATE TABLE IF NOT EXISTS {}", self.table_name);

        let fields_string = self
            .fields
            .iter()
            .map(get_field_query_string)
            .collect::<Vec<_>>()
            .join(",");

        let create_query = format!("{create_query} ({fields_string});");

        dbg!(&create_query);

        create_query
    }
}

fn get_field_query_string(field: &Field) -> String {
    match field {
        Field::Int { name, description } => {
            format!("{} BIGINT", name)
        }
    }
}
