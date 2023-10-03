/* SPDX-License-Identifier: MIT OR Apache-2.0 */
/* SPDX-FileCopyrightText: The Ferrocene Developers */

"use strict";

let toggleAnnotationMode = document.getElementById("toggle-annotation-mode");
let popup = document.getElementById("annotation-copied-popup");
let popupTimeout = null;

let refreshAnnotationMode = () => {
    if (toggleAnnotationMode.checked) {
        document.body.classList.add("annotation-mode");
    } else {
        document.body.classList.remove("annotation-mode");
    }
};
toggleAnnotationMode.addEventListener("change", refreshAnnotationMode);
refreshAnnotationMode();

document.querySelectorAll(".copiable").forEach(elem => {
    elem.addEventListener("click", (event => {
        // Only enable this when annotation mode is enabled.
        if (!toggleAnnotationMode.checked) {
            return;
        }
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
