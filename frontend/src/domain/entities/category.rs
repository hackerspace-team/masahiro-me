use std::fmt::Error;

#[derive(PartialEq, Clone, Debug)]
pub struct Category {
    id: i32,
    name: String,
}

impl Category {
    pub fn new(id: i32, name: &str) -> Result<Self, Error> {
        Ok(Category {
            id,
            name: name.to_string(),
        })
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
