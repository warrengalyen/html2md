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
    assert_eq!(md, "\nEXAMPLE\n==========\n[Dark Horizons](https://www.darkhorizons.com/)Movie News\n\nSynopsis\n----------\n")
} 

#[test]
fn test_list() {
    let md = parse_html(r#"<p><ul><li>Seven things has lady Lackless</li><li>Keeps them underneath her black dress</li><li>One a thing that's not for wearing</li></ul></p>"#);
    assert_eq!(md, r#"

* Seven things has lady Lackless
* Keeps them underneath her black dress
* One a thing that's not for wearing

"#)
} 

#[test]
fn test_list_formatted() {
    // let's use some some broken html
    let md = parse_html(r#"
    <ul><p>
        <li>You should NEVER see this error
            <ul>
                <li>Broken lines, broken strings
                <li>Broken threads, broken springs</li>
                <li>Broken idols, broken heads
                <li>People sleep in broken beds</li>
            </ul>
        </li>
        <li>Ain't no use jiving</li>
        <li>Ain't no use joking</li>
        <li>EVERYTHING IS BROKEN
    "#);
    assert_eq!(md, r#"

* You should NEVER see this error
    * Broken lines, broken strings
    * Broken threads, broken springs
    * Broken idols, broken heads
    * People sleep in broken beds
* Ain't no use jiving
* Ain't no use joking
* EVERYTHING IS BROKEN"#)
}

#[test]
fn test_quotes() {
    let md = parse_html("<p><blockquote>here's a quote\n next line of it</blockquote></p>");
    assert_eq!(md, "\n\n> here's a quote next line of it\n\n")
}

#[test]
fn test_quotes2() {
    let md = parse_html("<p><blockquote>here's<blockquote>nested quote!</blockquote> a quote\n next line of it</blockquote></p>");
    assert_eq!(md, r#"

> here's
> > nested quote!
>  a quote next line of it

"#)
}
 