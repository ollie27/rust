/*!
 * Copyright 2018 The Rust Project Developers. See the COPYRIGHT
 * file at the top-level directory of this distribution and at
 * http://rust-lang.org/COPYRIGHT.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

 /// <reference path="global.d.ts" />

/** @type {HTMLLinkElement} */
var currentTheme = document.getElementById("themeStyle");
/** @type {HTMLLinkElement} */
var mainTheme = document.getElementById("mainThemeStyle");

var savedHref = [];

/**
 * @param {any[] | NodeListOf<any>} arr
 * @param {(any) => any} func
 * @return {boolean}
 */
function onEach(arr, func) {
    if (arr && arr.length > 0 && func) {
        for (var i = 0; i < arr.length; i++) {
            if (func(arr[i]) === true) {
                return true;
            }
        }
    }
    return false;
}

/**
 * @param {string} name
 * @param {string} value
 */
function updateLocalStorage(name, value) {
    if (typeof(Storage) !== "undefined") {
        localStorage[name] = value;
    } else {
        // No Web Storage support so we do nothing
    }
}

/**
 * @param {string} name
 * @return {string}
 */
function getCurrentValue(name) {
    if (typeof(Storage) !== "undefined" && localStorage[name] !== undefined) {
        return localStorage[name];
    }
    return null;
}

/**
 * @param {HTMLLinkElement} styleElem
 * @param {HTMLLinkElement} mainStyleElem
 * @param {string} newTheme
 * @return {boolean}
 */
function switchTheme(styleElem, mainStyleElem, newTheme) {
    var fullBasicCss = "rustdoc" + resourcesSuffix + ".css";
    var fullNewTheme = newTheme + resourcesSuffix + ".css";
    var newHref = mainStyleElem.href.replace(fullBasicCss, fullNewTheme);

    if (styleElem.href === newHref) {
        return;
    }

    var found = false;
    if (savedHref.length === 0) {
        onEach(document.getElementsByTagName("link"), function(el) {
            savedHref.push(el.href);
        });
    }
    onEach(savedHref, function(el) {
        if (el === newHref) {
            found = true;
            return true;
        }
    });
    if (found === true) {
        styleElem.href = newHref;
        updateLocalStorage('rustdoc-theme', newTheme);
    }
}

switchTheme(currentTheme, mainTheme, getCurrentValue('rustdoc-theme') || 'light');
