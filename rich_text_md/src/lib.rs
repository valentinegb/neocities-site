use proc_macro::TokenStream;
use pulldown_cmark::{Event, Tag, TagEnd, TextMergeStream};
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
    let mut paragraph_just_ended = false;

    for event in iterator {
        if paragraph_just_ended {
            output += "\negui::RichText::new(\"\n\n\").append_to(&mut layout_job, ui.style(), egui::FontSelection::Default, egui::Align::Center);";
            paragraph_just_ended = false;
        }

        match event {
            Event::Start(tag) => current_tag = Some(tag),
            Event::End(tag) => {
                current_tag = None;

                if let TagEnd::Paragraph = tag {
                    paragraph_just_ended = true;
                }
            }
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
