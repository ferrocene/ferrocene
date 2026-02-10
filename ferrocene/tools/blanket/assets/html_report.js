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

function putBefore(path, section) {
    var path = details.querySelector("code").innerText;
    var before = Array.prototype.slice.call(section.children).find(function(e) {
        return e.querySelector("code").innerText > path;
    });

    if (before === null) {
        section.appendChild(details);
    } else {
        section.insertBefore(details, before);
    }
}

function main() {
    var currentQuery = "";
    var annotatedIsTested = false;

    function onSearch() {
        var numLinesTested = 0;
        var numLinesUntested = 0;

        for (section of document.querySelectorAll("section")) {
            var numFunctions = 0;
            for (details of section.querySelectorAll("details")) {
                var summary = details.querySelector("summary");
                if (summary.innerText.search(currentQuery) === -1) {
                    details.style.display = "none";
                } else {
                    details.style = "";

                    var testedLines = parseInt(summary.getAttribute("tested-lines"));
                    var untestedLines = parseInt(summary.getAttribute("untested-lines"));
                    var annotatedLines = parseInt(summary.getAttribute("annotated-lines"));

                    numLinesTested += testedLines;
                    numLinesUntested += untestedLines;
                    if (annotatedIsTested) {
                        numLinesTested += annotatedLines;
                    } else {
                        numLinesUntested += annotatedLines;
                    }
                    numFunctions += 1;
                }
            }
            section.querySelector(".count").textContent = numFunctions.toString();
            document.querySelector("button." + section.classList[0]).querySelector(".count").textContent = numFunctions.toString();
        }

        var totalLines = numLinesTested + numLinesUntested;
        var percentileLinesTested = 100.0;

        if (totalLines > 0) {
            percentileLinesTested = (numLinesTested / totalLines) * 100.0;
        }

        document.querySelector(".coverage-summary").children[0].textContent = `${percentileLinesTested.toFixed(2)}% (${numLinesTested}/${totalLines} lines)`;
    }

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
        annotatedIsTested = this.checked;

        var fullyTestedSection = document.querySelector("section.fully-tested").querySelector(".list");
        var partiallyTestedSection = document.querySelector("section.partially-tested").querySelector(".list");
        var fullyUntestedSection = document.querySelector("section.fully-untested").querySelector(".list");
        var fullyIgnoredSection = document.querySelector("section.fully-ignored").querySelector(".list");

        if (annotatedIsTested) {
            for (details of document.querySelectorAll("details.annotation")) {
                var path = details.querySelector("code").innerText;
                putBefore(path, fullyTestedSection);
            }
        } else {
            for (details of document.querySelectorAll("details.annotation")) {
                var path = details.querySelector("code").innerText;
                if (details.classList.contains("partially-tested")) {
                    putBefore(path, partiallyTestedSection);
                } else if (details.classList.contains("fully-untested")) {
                    putBefore(path, fullyUntestedSection);
                } else if (details.classList.contains("fully-tested")) {
                    putBefore(path, fullyTestedSection);
                } else {
                    putBefore(path, fullyIgnoredSection);
                }
            }
        }

        onSearch();
    });

    let searchBar = document.querySelector("input[name=search-bar]");
    searchBar.addEventListener("keydown", function(event) {
        if (event.key === "Enter") {
            currentQuery = searchBar.value;
            onSearch();
        }
    });
    let searchButton = document.querySelector("button[name=search-button]");
    searchButton.addEventListener("click", function(event) {
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
