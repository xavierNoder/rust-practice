enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}
struct User {
    name: String,
    age:u8,
    pub(crate) gender:Gender,
}

impl User {
    fn new(name: String, age: u8, gender: Gender) -> Self { Self { name, age, gender } }
}

impl Default for User {
    fn default() -> Self {
        User::new("Unknow User".into(),0, Gender::Unspecified)
    }
}