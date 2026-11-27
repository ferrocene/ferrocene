// format this file: `npx -y prettier -w html_report.js`

const summaryFullyTestedSelector = ".picker-buttons .fully-tested";
const summaryPartiallyTestedSelector = ".picker-buttons .partially-tested";
const summaryUntestedSelector = ".picker-buttons .fully-untested";
const summaryIgnoredSelector = ".picker-buttons .fully-ignored";
const summarySelectors = [
    summaryFullyTestedSelector,
    summaryPartiallyTestedSelector,
    summaryUntestedSelector,
    summaryIgnoredSelector,
];
const functionsSelector = ".functions";

function pathOf(details) {
    return details.querySelector("code").innerText;
}

function trueStatusOf(details) {
    if (details.classList.contains("partially-tested"))
        return "partially-tested";
    if (details.classList.contains("fully-untested")) return "fully-untested";
    if (details.classList.contains("fully-tested")) return "fully-tested";
    return "fully-ignored";
}

function mergeByPath(sorted, toInsert) {
    const sortedToInsert = toInsert.slice().sort((a, b) => {
        const pa = pathOf(a),
            pb = pathOf(b);
        return pa < pb ? -1 : pa > pb ? 1 : 0;
    });
    const merged = [];
    let i = 0,
        j = 0;
    while (i < sorted.length && j < sortedToInsert.length) {
        if (pathOf(sorted[i]) <= pathOf(sortedToInsert[j])) {
            merged.push(sorted[i++]);
        } else {
            merged.push(sortedToInsert[j++]);
        }
    }
    while (i < sorted.length) merged.push(sorted[i++]);
    while (j < sortedToInsert.length) merged.push(sortedToInsert[j++]);
    return merged;
}

let currentQuery = "";
let annotatedIsTested = false;

function onSearch() {
    const matcher = new RegExp(currentQuery);
    let numLinesTested = 0;
    let numLinesUntested = 0;
    let numBranchesTested = 0;
    let numBranchesUntested = 0;

    for (const section of document.querySelectorAll("section")) {
        let numFunctions = 0;
        for (const details of section.querySelectorAll("details")) {
            const summary = details.querySelector("summary");
            if (!matcher.test(summary.textContent)) {
                details.hidden = true;
            } else {
                details.hidden = false;

                const testedLines = parseInt(
                    summary.getAttribute("tested-lines"),
                );
                const untestedLines = parseInt(
                    summary.getAttribute("untested-lines"),
                );
                const annotatedLines = parseInt(
                    summary.getAttribute("annotated-lines"),
                );

                numLinesTested += testedLines;
                numLinesUntested += untestedLines;
                if (annotatedIsTested) {
                    numLinesTested += annotatedLines;
                } else {
                    numLinesUntested += annotatedLines;
                }

                const branchTested = parseInt(
                    summary.getAttribute("branch-tested"),
                );
                const branchUntested = parseInt(
                    summary.getAttribute("branch-untested"),
                );
                const branchAnnotated = parseInt(
                    summary.getAttribute("branch-annotated"),
                );

                numBranchesTested += branchTested;
                numBranchesUntested += branchUntested;
                if (annotatedIsTested) {
                    numBranchesTested += branchAnnotated;
                } else {
                    numBranchesUntested += branchAnnotated;
                }

                numFunctions += 1;
            }
        }
        section.querySelector(".count").textContent = numFunctions.toString();
        document
            .querySelector("button." + section.classList[0])
            .querySelector(".count").textContent = numFunctions.toString();
    }

    const totalLines = numLinesTested + numLinesUntested;
    let percentileLinesTested = 100.0;
    if (totalLines > 0) {
        percentileLinesTested = (numLinesTested / totalLines) * 100.0;
    }
    document.querySelector(".line-coverage").textContent =
        `${percentileLinesTested.toFixed(2)}% (${numLinesTested}/${totalLines} lines)`;

    const totalBranches = numBranchesTested + numBranchesUntested;
    let percentileBranchesTested = 100.0;
    if (totalBranches > 0) {
        percentileBranchesTested = (numBranchesTested / totalBranches) * 100.0;
    }
    document.querySelector(".branch-coverage").textContent =
        `${percentileBranchesTested.toFixed(2)}% (${numBranchesTested}/${totalBranches} branches)`;
}

function main() {
    const functionsElem = document.querySelector(functionsSelector);
    for (const selector of summarySelectors) {
        const summaryItemElem = document.querySelector(selector);
        const filter_listener = () => {
            if (
                functionsElem.dataset.filter == summaryItemElem.dataset.filter
            ) {
                delete functionsElem.dataset.filter;
            } else {
                functionsElem.dataset.filter = summaryItemElem.dataset.filter;
            }
        };
        summaryItemElem.addEventListener("click", filter_listener);
    }
    for (const elem of document.querySelectorAll(".line")) {
        elem.addEventListener("click", async (event) => {
            const filename = event.currentTarget.dataset.filename;
            const linenum = event.currentTarget.dataset.linenum;
            await navigator.clipboard.writeText(`${filename}:${linenum}`);
        });
    }
    const checkbox = document.querySelector("input[name=annotated-checkbox]");

    function applyAnnotatedAsTested(checked) {
        annotatedIsTested = checked;

        const listByStatus = {
            "fully-tested": document.querySelector(
                "section.fully-tested .list",
            ),
            "partially-tested": document.querySelector(
                "section.partially-tested .list",
            ),
            "fully-untested": document.querySelector(
                "section.fully-untested .list",
            ),
            "fully-ignored": document.querySelector(
                "section.fully-ignored .list",
            ),
        };

        const buckets = {
            "fully-tested": [],
            "partially-tested": [],
            "fully-untested": [],
            "fully-ignored": [],
        };
        for (const details of document.querySelectorAll("details.annotation")) {
            details.remove();
            const target = checked ? "fully-tested" : trueStatusOf(details);
            buckets[target].push(details);
        }

        for (const status in listByStatus) {
            if (buckets[status].length === 0) continue;
            const list = listByStatus[status];
            const merged = mergeByPath(
                Array.prototype.slice.call(list.children),
                buckets[status],
            );
            list.replaceChildren.apply(list, merged);
        }

        onSearch();
    }

    checkbox.addEventListener("change", function () {
        // Defer the DOM reshuffle to the next tick so the checkbox itself repaints
        // immediately instead of the tab appearing to hang while it runs.
        const checked = this.checked;
        setTimeout(() => applyAnnotatedAsTested(checked), 0);
    });

    const searchBar = document.querySelector("input[name=search-bar]");
    searchBar.addEventListener("keydown", (event) => {
        if (event.key === "Enter") {
            currentQuery = searchBar.value;
            onSearch();
        }
    });
    const searchButton = document.querySelector("button[name=search-button]");
    searchButton.addEventListener("click", () => {
        currentQuery = searchBar.value;
        onSearch();
    });

    onSearch();
}

if (document.readyState === "loading") {
    // Loading hasn't finished yet
    document.addEventListener("DOMContentLoaded", main);
} else {
    // `DOMContentLoaded` has already fired
    main();
}
