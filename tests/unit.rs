extern crate html2md;

use html2md::parse_html;

#[test]
fn test_dumb() {
    let md = parse_html("<p>AAAAAAAAAAAAA</p>");
    assert_eq!(md, "AAAAAAAAAAAAA")
}

#[test]
fn test_anchor() {
    let md = parse_html(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a></p>"#);
    assert_eq!(md, "[CLOCKWORK](https://mechanikadesign.com)")
}

#[test]
fn test_anchor2() {
    let md = parse_html(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
    assert_eq!(md, "[CLOCKWORK](https://mechanikadesign.com)[STEAM](https://www.mechanikadesign.com)")
}

#[test]
fn test_anchor3() {
    let md = parse_html(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><p/><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
    assert_eq!(md, "\
[CLOCKWORK](https://mechanikadesign.com)

[STEAM](https://www.mechanikadesign.com)")
}

#[test]
fn test_image() {
    let md = parse_html(r#"<p><a href="https://gitter.im/example/Lobby?utm_source=badge&amp;utm_medium=badge&amp;utm_campaign=pr-badge&amp;utm_content=badge"><img src="https://img.shields.io/gitter/room/example/example.svg" alt="Gitter"></a><br>"#);
    assert_eq!(md, "[![Gitter](https://img.shields.io/gitter/room/example/example.svg)](https://gitter.im/example/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)")
}

#[test]
fn test_escaping() {
    let md = parse_html(r#"<p>*god*'s in his **heaven** - all is right with the __world__</p>"#);
    assert_eq!(md, "\\*god\\*\'s in his \\*\\*heaven\\*\\* - all is right with the \\_\\_world\\_\\_")
}

#[test]
fn test_escaping_mid_hyphens() {
    let md = parse_html(r#"<h1>This is a header with-hyphen!</h1>"#);
    assert_eq!(md, "This is a header with-hyphen!\n==========")
}

#[test]
fn test_escaping_start_hyphens() {
    let md = parse_html(r#"<h1>- This is a header with starting hyphen!</h1>"#);
    assert_eq!(md, "\\- This is a header with starting hyphen!\n==========")
}

/// Note: Also strips multiple spaces
#[test]
fn test_escaping_start_hyphens_space() {
    let md = parse_html(r#"<h1>   - This is a header with starting hyphen!</h1>"#);
    assert_eq!(md, " \\- This is a header with starting hyphen!\n==========")
}

#[test]
fn test_headers() {
    let md = parse_html(r#"<h1 id="example">EXAMPLE</h1><p><a href="https://www.darkhorizons.com/">Dark Horizons</a>Movie News</p><h2 id="synopsis">Synopsis</h2>"#);
    assert_eq!(md, "\
EXAMPLE
==========

[Dark Horizons](https://www.darkhorizons.com/)Movie News

Synopsis
----------")
}

#[test]
fn test_escaping_start_equal() {
    let md = parse_html(r#"<p>This is NOT a header!<br/>===========</p>"#);
    assert_eq!(md, "This is NOT a header!  \n\\===========")
}

/// Note: Also strips multiple spaces
#[test]
fn test_escaping_start_equal_space() {
    let md = parse_html(r#"<p>This is NOT a header!<br/>  ===========</p>"#);
    assert_eq!(md, "This is NOT a header!  \n \\===========")
}

#[test]
fn test_escaping_start_hyphen() {
    let md = parse_html(r#"<p>This is NOT a header!<br/>-------</p>"#);
    assert_eq!(md, "This is NOT a header!  \n\\-------")
}

/// Note: Also strips multiple spaces
#[test]
fn test_escaping_start_hyphen_space() {
    let md = parse_html(r#"<p>This is NOT a header!<br/>     -------</p>"#);
    assert_eq!(md, "This is NOT a header!  \n \\-------")
} 
 