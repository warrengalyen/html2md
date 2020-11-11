use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::{Handle,NodeData};

#[derive(Default)]
pub(super) struct CodeHandler {
    code_type: String
}

impl CodeHandler {

    /// Used in both starting and finishing handling
    fn do_handle(&mut self, printer: &mut StructuredPrinter) {
        let immediate_parent = printer.parent_chain.last().unwrap().to_owned();
        if self.code_type == "code" && immediate_parent == "pre" {
            // we are already in "code" mode, just add newline
            printer.insert_newline();
            return;
        }

        match self.code_type.as_ref() {
            "pre" => printer.insert_str("```"),
            "code" | "samp" => printer.insert_str("`"),
            _ => {}
        }
    }
}

impl TagHandler for CodeHandler {

    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.code_type = match tag.data {
            NodeData::Element { ref name, .. } => name.local.to_string(),
        };

        self.do_handle(printer);
    }
    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        self.do_handle(printer);
    }
} 