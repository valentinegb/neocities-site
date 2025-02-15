use git2::Repository;
use proc_macro::TokenStream;

#[proc_macro]
pub fn commits(_item: TokenStream) -> TokenStream {
    let repository = Repository::open("./").unwrap();
    let mut revwalk = repository.revwalk().unwrap();
    let mut output = "vec![".to_string();

    revwalk.push_head().unwrap();

    for oid in revwalk {
        let commit = repository.find_commit(oid.unwrap()).unwrap();

        output += &format!(
            "Commit {{ date_time: chrono::DateTime::from_timestamp({}, 0).unwrap(), summary: rich_text_md::rich_text_md!({:?}), body: {} }},",
            commit.time().seconds(),
            commit.summary().unwrap(),
            match commit.body() {
                Some(body) => format!("Some(CommitBody {{ text: {body:?}, shown: false }})"),
                None => "None".to_string(),
            },
        );
    }

    output += "]";

    output.parse().unwrap()
}
