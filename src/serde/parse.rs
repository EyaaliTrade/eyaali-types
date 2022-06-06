pub fn parse<D, R>(value: D) -> Result<R, String>
where
    D: serde::Serialize,
    for<'r> R: serde::Deserialize<'r>,
{
    match serde_json::to_value(&value) {
        Ok(v) => {
            let document: R = match serde_json::from_value(v) {
                Ok(data) => data,
                Err(e) => return Err(e.to_string()),
            };
            Ok(document)
        }
        Err(e) => return Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        pub struct Entry {
            a: u64,
            b: u64,
            c: String,
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq)]
        pub struct Document {
            a: u64,
            b: u64,
            c: String,
        }
        let entry = Entry {
            a: 24,
            b: 42,
            c: "nice".to_string(),
        };

        assert_eq!(
            parse::<Entry, Document>(entry)?,
            Document {
                a: 24,
                b: 42,
                c: "nice".to_string(),
            }
        );
        Ok(())
    }
}
