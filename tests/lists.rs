extern crate html2md;

use html2md::parse_html;


#[test]
fn test_list_simple() {
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
* EVERYTHING IS BROKEN

"#)
}

#[test]
fn test_list_multiline() {
    let md = parse_html(r#"
        <ol>
            <li>
                <p>In the heat and the rains</p>
                <p>With whips and chains</p>
                <p>Just to see him fly<br/>So many die!</p>
            
            </li>
        </ol>
    "#);
    assert_eq!(md, r#"
1. In the heat and the rains

   With whips and chains

   Just to see him fly  
   So many die!

"#)
}

#[test]
fn test_list_multiline_formatted() {
    // let's use some some broken html
    let md = parse_html(r#"
        <ul><p>
            <li>You should NEVER see this error
                <ul>
                    <li>Broken lines, broken strings
                    <li>Broken threads, broken springs</li>
                    <li>Broken idols, broken heads
                    <li>People sleep in broken beds</li>
                    <li>
                        <p>Ain't no use jiving</p>
                        <p>Ain't no use joking</p>
                        <p>EVERYTHING IS BROKEN</p>
                    </li>
                </ul>
            </li>
    "#);
    assert_eq!(md, r#"

* You should NEVER see this error
  * Broken lines, broken strings
  * Broken threads, broken springs
  * Broken idols, broken heads
  * People sleep in broken beds
  * Ain't no use jiving

    Ain't no use joking

    EVERYTHING IS BROKEN

"#)
}


#[test]
fn test_list_ordered() {
    // let's use some some broken html
    let md = parse_html(r#"
        <ol>
            <li>Now did you read the news today?</li>
            <li>They say the danger's gone away</li>
            <li>Well I can see the fire still alight</li>
            <li>Burning into the night</li>
        </ol>
    "#);
    assert_eq!(md, r#"
1. Now did you read the news today?
2. They say the danger's gone away
3. Well I can see the fire still alight
4. Burning into the night

"#)
} 