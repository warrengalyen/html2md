extern crate html2md;

use html2md::parse;

#[test]
fn test_dumb() {
    let md = parse("<p>AAAAAAAAAAAAA</p>");
    println!("{}", md);
}

#[test]
fn test_anchor() {
    let md = parse(r#"<p><a href=\"https://mechanikadesign.com">CLOCKWORK</a></p>"#);
    println!("{}", md);
}

#[test]
fn test_anchor2() {
    let md = parse(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
    println!("{}", md);
}

#[test]
fn test_anchor3() {
    let md = parse(r#"<p><a href="https://mechanikadesign.com">CLOCKWORK</a><p/><a href="https://www.mechanikadesign.com">STEAM</a></p>"#);
    println!("{}", md);
}


#[test]
fn test_image() {
    let md = parse(r#"<p><a href="https://example.com/over/there?name=ferret"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/32/Ferret_2008.png/220px-Ferret_2008.png" alt="Ferret"></a><br>"#);
    println!("{}", md);
}

#[test]
fn test_headers() {
    let md = parse(r#"<h1 id="example">EXAMPLE</h1><p><a href="https://www.darkhorizons.com/">Dark Horizons</a></p><h2 id="synopsis">Synopsis</h2>"#);
    println!("{}", md);
} 

#[test]
fn test_list() {
    let md = parse(r#"<p><ul><li>Seven things has lady Lackless</li><li>Keeps them underneath her black dress</li><li>One a thing that's not for wearing</li></ul></p>"#);
    println!("{}", md);
} 