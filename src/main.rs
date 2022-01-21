#![feature(decl_macro)]
#[macro_use] extern crate rocket;

extern crate reqwest;
//use reqwest::blocking::Response;
use rocket::response::content::Html;
use rocket::http::RawStr;

extern crate url;
use url::Url;

extern crate serde;


struct Index
{
    name : String,
    url : Url
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubmedResponse {
    pub q: String,
    #[serde(rename = "num_hits")]
    pub num_hits: i64,
    pub hits: Vec<Hit>,
    //pub timings: Timings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    pub score: f64,
    pub doc: Doc,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub body: Vec<String>,
    pub journal: Option<Vec<String>>,
    pub title: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
    pub timings: Vec<Timing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    pub name: String,
    pub duration: i64,
    pub depth: i64,
}



#[get("/?<q>")]
fn index(q:Option<&RawStr>) -> Html<String>
{
    let indexes = vec![
        Index{name: String::from("pubmed"),   url : Url::parse("https://osf.creative-memory.eu/osf/api/0.0/index/cm/pubmed/en/tantivy/api/0.0").unwrap()},
        Index{name: String::from("wikipedia"), url : Url::parse("https://osf.creative-memory.eu/osf/api/0.0/index/cm/wikipedia/en/tantivy/api/0.0").unwrap()}
    ];

    let mut query_value = String::new();

    let mut results = String::new();
    let mut labels = String::new();
    if let Some(query) = q
    {
        let mut query_str = String::from("?q=");
        query_str.push_str(&query.html_escape());
        query_value.push_str(&query.html_escape());
        for (i, index) in indexes.iter().enumerate()
        {
            if let Ok(response) = reqwest::blocking::get(index.url.join(&query_str).unwrap().to_string())
            {
                if let Ok(json) = response.json::<PubmedResponse>()
                {
                    labels.push_str(&format!(r#"<label for="tab{}">{} {}</label>"#, i+1, index.name, json.num_hits));

                    let mut hits_list = String::new();
                    for hit in json.hits
                    {
                        let title = RawStr::from_str(hit.doc.title.first().unwrap());
                        let body = hit.doc.body.iter().map(|b| RawStr::from_str(b).html_escape()).collect::<Vec<_>>().join("<br>");
                        hits_list.push_str(&format!("<h4>{}</h4>{}<br>\n",
                                                    title.html_escape(),
                                                    body
                                                    ));
                    }
                    results.push_str(&format!(r#"<div class="tab{}">{hits_list}</div>"#, i + 1));
                }
                else
                {
                    results.push_str(&format!(r#"<div class="tab{}">no hits</div>"#, i + 1));
                }
            }
            else
            {
                results.push_str(&format!(r#"<div class="tab{}">no response from index</div>"#, i + 1));
            }
        }

    }
    Html(format!(
    r#"<html>
    <head>
    <title>OSF Search</title>
    <link href="https://osf.creative-memory.eu/osf_search/tabs.css" rel="stylesheet">
    </head>
    <body margin="5%">
        <h3>OSF Search</h3>
        <form action="" method="get" style="text-align:center;">
            <input name="q" id="q" type="text" width="400" value="{query_value}" />
            <input name="search" id="search" type="submit" value="search" />
        </form>

        <div class="tabbed">
           <input checked="checked" id="tab1" type="radio" name="tabs" />
           <input id="tab2" type="radio" name="tabs" />
           <input id="tab3" type="radio" name="tabs" />

           <nav>
             {labels}
           </nav>
           
           <figure>
             {results}
           </figure>
        </div>

    </body>
</html>
"#
    ))
}

//#[launch]
fn rocket() -> rocket::Rocket
{
    rocket::ignite().mount("/", routes![index])
}

fn main()
{
    rocket().launch();
}
