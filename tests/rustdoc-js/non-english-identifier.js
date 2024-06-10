const PARSED = [
    {
        query: '中文',
        elems: [{
            name: "中文",
            fullPath: ["中文"],
            pathWithoutLast: [],
            pathLast: "中文",
            generics: [],
            typeFilter: -1,
        }],
        returned: [],
        foundElems: 1,
        original: "中文",
        userQuery: "中文",
        error: null,
    },
    {
        query: '_0Mixed中英文',
        elems: [{
            name: "_0mixed中英文",
            fullPath: ["_0mixed中英文"],
            pathWithoutLast: [],
            pathLast: "_0mixed中英文",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "_0Mixed中英文",
        returned: [],
        userQuery: "_0mixed中英文",
        error: null,
    },
    {
        query: 'my_crate::中文API',
        elems: [{
            name: "my_crate::中文api",
            fullPath: ["my_crate", "中文api"],
            pathWithoutLast: ["my_crate"],
            pathLast: "中文api",
            generics: [],
            typeFilter: -1,
        }],
        foundElems: 1,
        original: "my_crate::中文API",
        returned: [],
        userQuery: "my_crate::中文api",
        error: null,
    },
    {
        query: '类型A,类型B<约束C>->返回类型<关联类型=路径::约束D>',
        elems: [{
            name: "类型a",
            fullPath: ["类型a"],
            pathWithoutLast: [],
            pathLast: "类型a",
            generics: [],
        }, {
            name: "类型b",
            fullPath: ["类型b"],
            pathWithoutLast: [],
            pathLast: "类型b",
            generics: [{
                name: "约束c",
                fullPath: ["约束c"],
                pathWithoutLast: [],
                pathLast: "约束c",
                generics: [],
            }],
        }],
        foundElems: 3,
        totalElems: 5,
        literalSearch: true,
        original: "类型A,类型B<约束C>->返回类型<关联类型=路径::约束D>",
        returned: [{
            name: "返回类型",
            fullPath: ["返回类型"],
            pathWithoutLast: [],
            pathLast: "返回类型",
            generics: [],
        }],
        userQuery: "类型a,类型b<约束c>->返回类型<关联类型=路径::约束d>",
        error: null,
    },
    {
        query: 'my_crate 中文宏!',
        elems: [{
            name: "my_crate 中文宏",
            fullPath: ["my_crate", "中文宏"],
            pathWithoutLast: ["my_crate"],
            pathLast: "中文宏",
            generics: [],
            typeFilter: 16,
        }],
        foundElems: 1,
        original: "my_crate 中文宏!",
        returned: [],
        userQuery: "my_crate 中文宏!",
        error: null,
    },
    {
        query: '非法符号——',
        elems: [],
        foundElems: 0,
        original: "非法符号——",
        returned: [],
        userQuery: "非法符号——",
        error: "Unexpected `—` after `号` (not a valid identifier)",
    }
]
const EXPECTED = [
    {
        query: '加法',
        others: [
            {
                name: "add",
                path: "non_english_identifier",
                is_alias: true,
                alias: "加法",
                href: "../non_english_identifier/macro.add.html"
            },
            {
                name: "add",
                path: "non_english_identifier",
                is_alias: true,
                alias: "加法",
                href: "../non_english_identifier/fn.add.html"
            },
            {
                name: "加法",
                path: "non_english_identifier",
                href: "../non_english_identifier/trait.加法.html",
                desc: "Add"
            },
            {
                name: "中文名称的加法宏",
                path: "non_english_identifier",
                href: "../non_english_identifier/macro.中文名称的加法宏.html",
            },
            {
                name: "中文名称的加法API",
                path: "non_english_identifier",
                href: "../non_english_identifier/fn.中文名称的加法API.html",
            }],
        in_args: [{
            name: "加上",
            path: "non_english_identifier::加法",
            href: "../non_english_identifier/trait.加法.html#tymethod.加上",
        }],
        returned: [],
    },
    { // Extensive type-based search is still buggy, experimental & work-in-progress.
        query: '可迭代->可选',
        others: [{
            name: "总计",
            path: "non_english_identifier",
            href: "../non_english_identifier/fn.总计.html",
            desc: "“sum”"
        }],
        in_args: [],
        returned: [],
    },
];
