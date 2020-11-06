extern crate html2md;

use html2md::parse;

#[test]
fn test() {
    parse("<p>aaaaa</p>");
}

#[test]
fn test_anchor() {
    parse(r#"<p><a href=\"https://mechanikadesign.com">CLOCKWORK</a></p>"#);
}

#[test]
fn test_anchor2() {
    parse(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
}

#[test]
fn test_anchor3() {
    parse(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><p/><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
}


#[test]
fn test_image() {
    parse(r#"<p><a href="https://example.com/over/there?name=ferret"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Ferret_2008.png/220px-Ferret_2008.png" alt="Ferret"></a><br>"#);
}


#[test]
fn test_headers() {
    parse(r#"<h1 id="example">EXAMPLE</h1><p><a href="https://www.darkhorizons.com/">Dark Horizons</a></p><h2 id="synopsis">Synopsis</h2>"#);
} 