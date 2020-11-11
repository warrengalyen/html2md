use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::Handle;

#[derive(Default)]
pub(super) struct QuoteHandler {
    start_pos: usize
}

impl TagHandler for QuoteHandler {

    fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.position;
        printer.insert_newline();
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        // replace all new lines with newline + >
        let quote = "> ";
        let mut index = printer.data.len();
        while index > self.start_pos {
            if printer.data.as_bytes().iter().nth(index) == Some(&b'\n') {
                printer.data.insert_str(index + 1, &quote);
                printer.position += quote.len();
            }
            index -= 1;
        }

        printer.data.insert_str(self.start_pos + 1, &quote);
        printer.position += quote.len();

        printer.insert_newline();
        printer.insert_newline();
    }
}