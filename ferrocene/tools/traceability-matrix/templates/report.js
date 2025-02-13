/* SPDX-License-Identifier: MIT OR Apache-2.0 */
/* SPDX-FileCopyrightText: The Ferrocene Developers */

"use strict";

let popup = document.getElementById("annotation-copied-popup");
let popupTimeout = null;

document.querySelectorAll(".copiable").forEach(elem => {
    elem.addEventListener("click", (event => {
        event.preventDefault();

        let text = "// ferrocene-annotations: " + elem.innerText + "\n";
        if ("data-copy-title" in elem.attributes) {
            text += "// " + elem.attributes["data-copy-title"].value + "\n";
        }

        navigator
            .clipboard
            .writeText(text)
            .then(() => {
                // Show the popup and hide it after a bit.
                if (popupTimeout !== null) {
                    clearTimeout(popupTimeout);
                    popupTimeout = null;
                }
                popup.classList.remove("hidden");
                popupTimeout = setTimeout(() => {
                    popup.classList.add("hidden");
                    popupTimeout = null;
                }, 1000); // 1s
            })
            .catch(err => console.log("Failed to copy from the clipboard: " + err));

    }).bind(elem));
});

function detailsButton(selector, setOpenTo) {
    document.querySelectorAll(selector).forEach(button => {
        button.addEventListener("click", event => {
            document.querySelectorAll("details").forEach(details => details.open = setOpenTo);
        });
    });
}
detailsButton(".expand-all", true);
detailsButton(".collapse-all", false);
