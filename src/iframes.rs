use lazy_static::lazy_static;

use super::TagHandler;
use super::StructuredPrinter;

use crate::common::get_tag_attr;

use regex::Regex;
use html5ever::rcdom::Handle;

lazy_static! {
    /// Pattern that detects iframes with Youtube embedded videos
    /// Examples: 
    /// * `https://www.youtube.com/embed/zE-dmXZp3nU?wmode=opaque`
    /// * `https://www.youtube.com/embed/5yo6exIypkY`
    /// * `https://www.youtube.com/embed/TXm6IXrbQuM`
    static ref YOUTUBE_PATTERN : Regex = Regex::new(r"www\.youtube\.com/embed/([-\w]+)").unwrap(); 

    /// Pattern that detects iframes with Instagram embedded photos
    /// Examples:
    /// * `https://www.instagram.com/p/B1BKr9Wo8YX/embed/`
    /// * `https://www.instagram.com/p/BpKjlo-B4uI/embed/`
    static ref INSTAGRAM_PATTERN: Regex = Regex::new(r"www\.instagram\.com/p/([-\w]+)/embed").unwrap();
}

#[derive(Default)]
pub(super) struct IframeHandler {
}

/// We currently support only Youtube iframes
impl TagHandler for IframeHandler {

    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();

        let src = get_tag_attr(tag, "src");
        //let width = get_tag_attr(tag, "width");
        //let height = get_tag_attr(tag, "height");

        if src == None {
            return;
        }

        let src = src.unwrap();

        if let Some(capture) = YOUTUBE_PATTERN.captures(&src) {
            let media_id = capture.get(1).unwrap();
            printer.append_str(&format!("[![Embedded YouTube video](https://img.youtube.com/vi/{mid}/0.jpg)](https://www.youtube.com/watch?v={mid})", mid = media_id.as_str()));
            return
        }

        if let Some(capture) = INSTAGRAM_PATTERN.captures(&src) {
            let media_id = capture.get(1).unwrap();
            printer.append_str(&format!("[![Embedded Instagram post](https://www.instagram.com/p/{mid}/media/?size=m)](https://www.instagram.com/p/{mid}/embed/)", mid = media_id.as_str()));
            return
        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_newline();
        printer.insert_newline();
    }
} 