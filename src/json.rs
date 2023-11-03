enum Gender {
    Male,
    Female,
    Diverse,
}

struct Employer {
    vorname: String,
    nachname: String,
    alter: u32,
    gender: Gender,
}

#[cfg(test)]
mod tests {
    use crate::json::{Employer, Gender};

    #[test]
    fn employer_can_be_serialized_to_json() {
        // given
        let employer = Employer {
            vorname: "Rust".to_string(),
            nachname: "Acean".to_string(),
            alter: 8,
            gender: Gender::Diverse,
        };

        // when (use the below commented line - if you added serde dependencies and did the needed things ;))
        // let serialized_json = serde_json::to_string(&employer).unwrap();
        let serialized_json = "abc";

        // then
        let expected_string = r#"{"vorname":"Rust","nachname":"Acean","alter":8,"gender":"Diverse"}"#;
        assert_eq!(serialized_json, expected_string);
    }
}