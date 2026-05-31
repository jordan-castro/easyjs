#[cfg(test)]
mod tests {
    #[test]
    fn test_objects() {
        let src = r#"
assert := import("testing").assert

Person := class {
  name: string
  age: int
}

p := Person('Jordan', 24)
assert(p.name == 'Jordan')
assert(p.age == 24)
assert(Person.name == "")
assert(Person.age == 0)
        "#;
    }
}