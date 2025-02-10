# public-docs warning

This directory contains the implementation of the warning banner we show in
[public-docs.ferrocene.dev]. It is built with two components:

* `header.html` is the template we include in all of our documentation (even
  when building documentation not meant for public-docs!). Its content is hidden
  with `display: none;`, so that it doesn't *actually* show up.

* `public-docs-warning.css` is the styling of the warning, and crucially it is
  only included when uploading the documentation to public-docs. This results in
  the banner only being displayed there, even though we always include the HTML.

To show the warning locally, run this command:

```
ln -s ../../../ferrocene/doc/public-docs-warning/public-docs-warning.css build/host/doc/
```

[public-docs.ferrocene.dev]: https://public-docs.ferrocene.dev
