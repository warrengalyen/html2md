use super::TagHandler;
use super::StructuredPrinter;

use html5ever::rcdom::NodeData;

pub struct ImgHandler {

}

impl TagHandler for ImgHandler {

    fn before_handle(&mut self, _printer: &mut StructuredPrinter) {

    }

    fn handle(&mut self, tag: &NodeData, printer: &mut StructuredPrinter) {
        // try to extract a hyperlink
        let url = match tag {
            &NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();
                let src = attrs.iter().find(|attr| attr.name.local.to_string() == "src");
                match src {
                    Some(link) => link.value.to_string(),
                    None => String::new()
                }
            }
            _ => String::new()
        };

        // at this point we know it's an anchor tag
        printer.data.insert_str(printer.position, format!("![]({})", url).as_ref());

        // inserted a link, now we have to update position to move it one point forward, after "[" sign
        printer.position += 3
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.position = printer.data.len();
    }

    fn is_applicable(&self, tag_name: String) -> bool {
        return tag_name == "img";
    }
}