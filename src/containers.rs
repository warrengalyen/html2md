use super::TagHandler;
use super::StructuredPrinter;

use markup5ever_rcdom::Handle;

#[derive(Default)]
pub(super) struct ContainerHandler;

impl TagHandler for ContainerHandler {

    fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();
    }
} 