use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

#[derive(Default)]
pub struct StyleHandler {
    style_type: String
}

impl TagHandler for StyleHandler {

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        self.style_type = match tag {
            &NodeData::Element { ref name, .. } => name.local.to_string(),
            _ => String::new()
        };

        // staring markup
        match self.style_type.as_ref() {
            "b" | "strong" => printer.insert_str("**"), // bold
            "i" | "em" => printer.insert_str("*"),      // italic
            "s" | "del" => printer.insert_str("~~"),    // strikethrough
            "u" | "ins" => printer.insert_str("__"),    // underline
            _ => {}
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // finishing markup
        match self.style_type.as_ref() {
            "b" | "strong" => printer.insert_str("**"),
            "i" | "em" => printer.insert_str("*"),
            "s" | "del" => printer.insert_str("~~"),
            "u" | "ins" => printer.insert_str("__"),
            _ => {}
        }
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "b" || tag_name == "i" || tag_name == "s" || tag_name == "u" || tag_name == "strong" || tag_name == "em" || tag_name == "del" || tag_name == "ins";
    }
}