// ignore-tidy-cr
// ignore-tidy-tab

fn main() {
    // these literals are just silly.
    ''';
    //~^ ERROR: character constant must be escaped: `'`

    // note that this is a literal "\n" byte
    '
';
    //~^^ ERROR: character constant must be escaped: `\n`

    // note that this is a literal "\r" byte
    ''; //~ ERROR: character constant must be escaped: `\r`

    // note that this is a literal NULL
    '- -'; //~ ERROR: character literal may only contain one codepoint

    // note that this is a literal tab character here
    '	';
    //~^ ERROR: character constant must be escaped: `\t`
}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
