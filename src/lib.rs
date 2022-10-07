use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(name: &str) -> String{
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub struct Person {
    age: u32,
    name: String,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(age: u32, name: String) -> Self {
        Self { age, name }
    }

    pub fn info(&self) -> String {
        format!("{} age is {}", self.name, self.age)
    }
}