const summaryFullyTestedSelector = ".picker-buttons .fully-tested"
const summaryPartiallyTestedSelector = ".picker-buttons .partially-tested"
const summaryUntestedSelector = ".picker-buttons .fully-untested"
const summaryIgnoredSelector = ".picker-buttons .fully-ignored"
const summarySelectors = [
    summaryFullyTestedSelector,
    summaryPartiallyTestedSelector,
    summaryUntestedSelector,
    summaryIgnoredSelector,
];
let functionsSelector = ".functions"

function onSearch(query) {
    var numLinesTested = 0;
    var numLinesUntested = 0;

    for (section of document.querySelectorAll("section")) {
        var numFunctions = 0;
        for (details of section.querySelectorAll("details")) {
            var summary = details.querySelector("summary");
            if (summary.innerText.search(query) === -1) {
                summary.style.display = "none";
            } else {
                summary.style = "";

                var testedLines = parseInt(summary.getAttribute("tested-lines"));
                var untestedLines = parseInt(summary.getAttribute("untested-lines"));
                var annotatedLines = parseInt(summary.getAttribute("annotated-lines"));

                numLinesTested += testedLines;
                numLinesUntested += untestedLines + annotatedLines;
                numFunctions += 1;
            }
        }
        section.querySelector(".count").textContent = numFunctions.toString();
    }

    var totalLines = numLinesTested + numLinesUntested;
    var percentileLinesTested = (numLinesTested / totalLines) * 100.0;

    document.querySelector(".coverage-summary").children[0].textContent = `${percentileLinesTested.toFixed(2)}% (${numLinesTested}/${totalLines} lines)`;
}

function main() {
    let functionsElem = document.querySelector(functionsSelector);
    for (selector of summarySelectors) {
        let summaryItemElem = document.querySelector(selector);
        let filter_listener = _ => {
            if (functionsElem.dataset.filter == summaryItemElem.dataset.filter) {
                delete(functionsElem.dataset.filter);
            } else {
                functionsElem.dataset.filter = summaryItemElem.dataset.filter;
            }
        }
        summaryItemElem.addEventListener("click", filter_listener);
    }
    for (elem of document.querySelectorAll(".line")) {
        elem.addEventListener("click", async event => {
            let filename = event.target.dataset.filename;
            let linenum = event.target.dataset.linenum;
            await navigator.clipboard.writeText(`${filename}:${linenum}`);
        });
    }
    let checkbox = document.querySelector("input[name=annotated-checkbox]");

    checkbox.addEventListener('change', function() {
        var r = document.querySelector(':root');
        var rs = getComputedStyle(r);
        if (this.checked) {
            let color = rs.getPropertyValue("--var-ignored");
            r.style.setProperty("--var-untested-annotated", color);
            r.style.setProperty("--var-partial-annotated", color);

            r.style.setProperty("--var-annotated-text", "line-through");
        } else {
            let color_untested = rs.getPropertyValue("--var-untested");
            r.style.setProperty("--var-untested-annotated", color_untested);

            let color_partial = rs.getPropertyValue("--var-partial");
            r.style.setProperty("--var-partial-annotated", color_partial);

            r.style.setProperty("--var-annotated-text", null);
        }
    });

    let searchBar = document.querySelector("input[name=search-bar]");
    searchBar.addEventListener("keydown", function(event) {
        if (event.key === "Enter") {
            let query = searchBar.value;
            onSearch(query);
        }
    });
    let searchButton = document.querySelector("button[name=search-button]");
    searchButton.addEventListener("onClick", function(event) {
        let query = searchBar.value;
        onSearch(query);
    });

    onSearch(".");
}

if (document.readyState === "loading") {
  // Loading hasn't finished yet
  document.addEventListener("DOMContentLoaded", main);
} else {
  // `DOMContentLoaded` has already fired
  main();
}
