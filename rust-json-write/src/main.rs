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

fn main (){
    let article: Article = Article {
        article: String::from("how to write with json in Rust"),
        author: String::from("relaxandlaidback"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence")
            },
            Paragraph {
                name: String::from("body of the article")
            },
            Paragraph {
                name: String::from("end of paragraph")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("the JSON is: {}", json)
}