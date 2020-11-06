extern crate html2md

use html2md::parse;

#[test]
fn test() {
    parse("<p>aaaaa</p>");
}

#[test]
fn test_anchor() {
    parse("<p><a href=\"https://mechanikadesign.com\">mumumu</a></p>");
}