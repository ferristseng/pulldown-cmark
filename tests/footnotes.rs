// This file is auto-generated by the build script
// Please, do not modify it manually

extern crate pulldown_cmark;

include!("normalize_html.rs.inc");


    #[test]
    fn footnotes_test_1() {
        let original = r##"Lorem ipsum.[^a]

[^a]: Cool.
"##;
        let expected = r##"<p>Lorem ipsum.<sup class="footnote-reference"><a href="#a">1</a></sup></p>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
<p>Cool.</p>
</div>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_2() {
        let original = r##"> This is the song that never ends.\
> Yes it goes on and on my friends.[^lambchops]
>
> [^lambchops]: <https://www.youtube.com/watch?v=0U2zJOryHKQ>
"##;
        let expected = r##"<blockquote>
<p>This is the song that never ends.<br />
Yes it goes on and on my friends.<sup class="footnote-reference"><a href="#lambchops">1</a></sup></p>
<div class="footnote-definition" id="lambchops"><sup class="footnote-definition-label">1</sup>
<p><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">https://www.youtube.com/watch?v=0U2zJOryHKQ</a></p>
</div>
</blockquote>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_3() {
        let original = r##"Songs that simply loop are a popular way to annoy people. [^examples]

[^examples]:
 * [The song that never ends](https://www.youtube.com/watch?v=0U2zJOryHKQ)
 * [I know a song that gets on everybody's nerves](https://www.youtube.com/watch?v=TehWI09qxls)
 * [Ninety-nine bottles of beer on the wall](https://www.youtube.com/watch?v=qVjCag8XoHQ)
"##;
        let expected = r##"<p>Songs that simply loop are a popular way to annoy people. <sup class="footnote-reference"><a href="#examples">1</a></sup></p>
<div class="footnote-definition" id="examples"><sup class="footnote-definition-label">1</sup>
<ul>
<li><a href="https://www.youtube.com/watch?v=0U2zJOryHKQ">The song that never ends</a></li>
<li><a href="https://www.youtube.com/watch?v=TehWI09qxls">I know a song that gets on everybody's nerves</a></li>
<li><a href="https://www.youtube.com/watch?v=qVjCag8XoHQ">Ninety-nine bottles of beer on the wall</a></li>
</ul>
</div>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_4() {
        let original = r##"[^lorem]: If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.

I had largely given over my inquiries into what Professor Angell called the "Cthulhu Cult", and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.
"##;
        let expected = r##"<div class="footnote-definition" id="lorem"><sup class="footnote-definition-label">1</sup>
<p>If heaven ever wishes to grant me a boon, it will be a total effacing of the results of a mere chance which fixed my eye on a certain stray piece of shelf-paper. It was nothing on which I would naturally have stumbled in the course of my daily round, for it was an old number of an Australian journal, the Sydney Bulletin for April 18, 1925. It had escaped even the cutting bureau which had at the time of its issuance been avidly collecting material for my uncle's research.</p>
</div>
<p>I had largely given over my inquiries into what Professor Angell called the &quot;Cthulhu Cult&quot;, and was visiting a learned friend in Paterson, New Jersey; the curator of a local museum and a mineralogist of note. Examining one day the reserve specimens roughly set on the storage shelves in a rear room of the museum, my eye was caught by an odd picture in one of the old papers spread beneath the stones. It was the Sydney Bulletin I have mentioned, for my friend had wide affiliations in all conceivable foreign parts; and the picture was a half-tone cut of a hideous stone image almost identical with that which Legrasse had found in the swamp.</p>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_5() {
        let original = r##"[^ipsum]: How much wood would a woodchuck chuck.

If a woodchuck could chuck wood.


# Forms of entertainment that aren't childish
"##;
        let expected = r##"<div class="footnote-definition" id="ipsum"><sup class="footnote-definition-label">1</sup>
<p>How much wood would a woodchuck chuck.</p>
</div>
<p>If a woodchuck could chuck wood.</p>
<h1>Forms of entertainment that aren't childish</h1>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_6() {
        let original = r##"> He's also really stupid. [^why]
>
> [^why]: Because your mamma!

As such, we can guarantee that the non-childish forms of entertainment are probably more entertaining to adults, since, having had a whole childhood doing the childish ones, the non-childish ones are merely the ones that haven't gotten boring yet.
"##;
        let expected = r##"<blockquote>
<p>He's also really stupid. <sup class="footnote-reference"><a href="#why">1</a></sup></p>
<div class="footnote-definition" id="why"><sup class="footnote-definition-label">1</sup>
<p>Because your mamma!</p>
</div>
</blockquote>
<p>As such, we can guarantee that the non-childish forms of entertainment are probably more entertaining to adults, since, having had a whole childhood doing the childish ones, the non-childish ones are merely the ones that haven't gotten boring yet.</p>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_7() {
        let original = r##"Nested footnotes are considered poor style. [^a] [^xkcd]

[^a]: This does not mean that footnotes cannot reference each other. [^b]

[^b]: This means that a footnote definition cannot be directly inside another footnote definition.
> This means that a footnote cannot be directly inside another footnote's body. [^e]
>
> [^e]: They can, however, be inside anything else.

[^xkcd]: [The other kind of nested footnote is, however, considered poor style.](https://xkcd.com/1208/)
"##;
        let expected = r##"<p>Nested footnotes are considered poor style. <sup class="footnote-reference"><a href="#a">1</a></sup> <sup class="footnote-reference"><a href="#xkcd">2</a></sup></p>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
<p>This does not mean that footnotes cannot reference each other. <sup class="footnote-reference"><a href="#b">3</a></sup></p>
</div>
<div class="footnote-definition" id="b"><sup class="footnote-definition-label">3</sup>
<p>This means that a footnote definition cannot be directly inside another footnote definition.</p>
<blockquote>
<p>This means that a footnote cannot be directly inside another footnote's body. <sup class="footnote-reference"><a href="#e">4</a></sup></p>
<div class="footnote-definition" id="e"><sup class="footnote-definition-label">4</sup>
<p>They can, however, be inside anything else.</p>
</div>
</blockquote>
</div>
<div class="footnote-definition" id="xkcd"><sup class="footnote-definition-label">2</sup>
<p><a href="https://xkcd.com/1208/">The other kind of nested footnote is, however, considered poor style.</a></p>
</div>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }

    #[test]
    fn footnotes_test_8() {
        let original = r##"[^Doh] Ray Me Fa So La Te Do! [^1]

[^Doh]: I know. Wrong Doe. And it won't render right.
[^1]: Common for people practicing music.
"##;
        let expected = r##"<p><sup class="footnote-reference"><a href="#Doh">1</a></sup> Ray Me Fa So La Te Do! <sup class="footnote-reference"><a href="#1">2</a></sup></p>
<div class="footnote-definition" id="Doh"><sup class="footnote-definition-label">1</sup>
<p>I know. Wrong Doe. And it won't render right.
<sup class="footnote-reference"><a href="#1">2</a></sup>: Common for people practicing music.</p>
</div>
"##;

        use pulldown_cmark::{Parser, html, Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};

        let mut s = String::new();

        let mut opts = Options::empty();
        opts.insert(OPTION_ENABLE_TABLES);
        opts.insert(OPTION_ENABLE_FOOTNOTES);

        let p = Parser::new_ext(&original, opts);
        html::push_html(&mut s, p).unwrap();

        assert_eq!(normalize_html(&expected), normalize_html(&s));
    }