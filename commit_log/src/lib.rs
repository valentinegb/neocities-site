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
            "(chrono::DateTime::from_timestamp({}, 0).unwrap(), rich_text_md::rich_text_md!({:?})),",
            commit.time().seconds(),
            commit.message().unwrap(),
        );
    }

    output += "]";

    output.parse().unwrap()
}
