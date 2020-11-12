extern crate html2md;

use html2md::parse_html;

#[test]
fn test_dumb() {
    let md = parse_html("<p>AAAAAAAAAAAAA</p>");
    assert_eq!(md, "AAAAAAAAAAAAA\n\n")
}

#[test]
fn test_anchor() {
    let md = parse_html(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a></p>"#);
    assert_eq!(md, "[CLOCKWORK](https://mechanikadesign.com)\n\n")
}

#[test]
fn test_anchor2() {
    let md = parse_html(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
    assert_eq!(md, "[CLOCKWORK](https://mechanikadesign.com)[STEAM](https://www.mechanikadesign.com)\n\n")
}

#[test]
fn test_anchor3() {
    let md = parse_html(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><p/><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
    assert_eq!(md, "[CLOCKWORK](https://mechanikadesign.com)\n\n[STEAM](https://www.mechanikadesign.com)\n\n")
}

#[test]
fn test_escaping() {
    let md = parse_html(r#"<p>*god*'s in his **heaven** - all is right with the __world__</p>"#);
    assert_eq!(md, "\\*god\\*\'s in his \\*\\*heaven\\*\\* - all is right with the \\_\\_world\\_\\_\n\n")
}

#[test]
fn test_image() {
    let md = parse_html(r#"<p><a href="https://example.com/over/there?name=ferret"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Ferret_2008.png/220px-Ferret_2008.png" alt="Ferret"></a><br>"#);
    assert_eq!(md, "[![Ferret](https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Ferret_2008.png/220px-Ferret_2008.png)](https://example.com/over/there?name=ferret)  \n\n")
}

#[test]
fn test_headers() {
    let md = parse_html(r#"<h1 id="example">EXAMPLE</h1><p><a href="https://www.darkhorizons.com/">Dark Horizons</a>Movie News</p><h2 id="synopsis">Synopsis</h2>"#);
    assert_eq!(md, "\n\nEXAMPLE\n==========\n[Dark Horizons](https://www.darkhorizons.com/)Movie News\n\nSynopsis\n----------\n")
}


 