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
}

if (document.readyState === "loading") {
  // Loading hasn't finished yet
  document.addEventListener("DOMContentLoaded", main);
} else {
  // `DOMContentLoaded` has already fired
  main();
}