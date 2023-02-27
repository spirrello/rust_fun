use std::default;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
fn main() {
    let mut fields_builder = FieldsBuilder::default();
    let mut field_item_builder = FieldItemBuilder::default();

    let field1 = field_item_builder
        .title("hola".to_owned())
        .value("yay".to_owned())
        .build();

    let field2 = field_item_builder
        .short(true)
        .title("sup".to_owned())
        .value("son".to_owned())
        .build();

    fields_builder.add_fields(field1).add_fields(field2);

    for x in fields_builder.fields.iter() {
        println!("{:?}", x);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldItem {
    pub short: Option<bool>,
    pub title: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldItemBuilder {
    pub short: Option<bool>,
    pub title: Option<String>,
    pub value: Option<String>,
}

impl FieldItemBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn short(&mut self, short: bool) -> &mut Self {
        self.short = Some(short);
        self
    }
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }
    pub fn value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
    fn build(&mut self) -> FieldItem {
        FieldItem {
            short: Some(self.short.clone().unwrap_or_default()),
            title: self.title.clone().unwrap(),
            value: self.value.clone().unwrap(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
struct FieldsBuilder {
    #[builder(setter(custom))]
    pub fields: Vec<FieldItem>,
}

impl FieldsBuilder {
    fn add_fields(&mut self, field: FieldItem) -> &mut Self {
        self.fields.push(field);
        self
    }
}
