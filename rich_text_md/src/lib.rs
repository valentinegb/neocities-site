use proc_macro::TokenStream;
use pulldown_cmark::{Event, Tag, TextMergeStream};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn rich_text_md(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as LitStr).value();
    let iterator = TextMergeStream::new(pulldown_cmark::Parser::new_ext(
        &input,
        pulldown_cmark::Options::ENABLE_SUBSCRIPT,
    ));
    let mut output = "{\nuse crate::fonts::RichTextExt as _;\n\nlet mut layout_job = egui::text::LayoutJob::default();\n".to_string();
    let mut current_tag = None;

    for event in iterator {
        match event {
            Event::Start(tag) => current_tag = Some(tag),
            Event::End(_) => current_tag = None,
            Event::Text(text) => {
                output += &format!("\negui::RichText::new(format!({:?}))", &*text);

                if let Some(current_tag) = &current_tag {
                    match current_tag {
                        Tag::Strong => output += ".proportional_bold()",
                        Tag::Emphasis => output += ".proportional_italics()",
                        Tag::Subscript => output += ".proportional_thin()",
                        _ => (),
                    }
                }

                output += ".append_to(&mut layout_job, ui.style(), egui::FontSelection::Default, egui::Align::Center);";
            }
            _ => (),
        }
    }

    output += "\n\nlayout_job\n}";

    output.parse().unwrap()
}
