const summaryFullyTestedSelector = ".summary .fully-tested"
const summaryPartiallyTestedSelector = ".summary .partially-tested"
const summaryUntestedSelector = ".summary .fully-untested"
const summaryIgnoredSelector = ".summary .fully-ignored"
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
        summaryItemElem.addEventListener("click", _ => {
            functionsElem.dataset.filter = summaryItemElem.dataset.filter;
        });
    }
    document.querySelector("#reset").addEventListener("click", _ => {
        functionsElem.dataset.filter = null;
    });
}

if (document.readyState === "loading") {
  // Loading hasn't finished yet
  document.addEventListener("DOMContentLoaded", main);
} else {
  // `DOMContentLoaded` has already fired
  main();
}