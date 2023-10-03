// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

// Value injected by signature_page.py.
const signed = %%%SIGNED%%%;

let validElement = document.getElementById("breadcrumbs-valid-signature");
let invalidElement = document.getElementById("breadcrumbs-invalid-signature");

if (signed) {
    validElement.classList.remove("hidden");
    invalidElement.classList.add("hidden");
} else {
    validElement.classList.add("hidden");
    invalidElement.classList.remove("hidden");
}
