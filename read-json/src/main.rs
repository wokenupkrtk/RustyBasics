use serde::{Deserialize, Serialize};

//noinspection ALL,RsUnresolvedPath
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

//noinspection ALL,RsUnresolvedPath
#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() -> Result<(), serde_json::Error> {
    let json = r#"
{
  "article": "How to read a json file with a rust program",
  "author": "relaxandlaidback",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
"#;
    let parsed: Article = read_json_typed(json);

    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);


    Ok(())
}


fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed

}