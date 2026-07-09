/*!
 * docmd (v0.8.10)
 * Copyright (c) 2025-present docmd.io
 * License: MIT
 */
(function(){function n(){var e=document.querySelector(".sidebar-menu-button"),a=document.querySelector(".sidebar"),t=document.querySelector(".sidebar-nav");if(!(!e||!a||e.dataset.docslimeEnhanced==="true")){t&&!t.id&&(t.id="docslime-sidebar-nav"),e.dataset.docslimeEnhanced="true",e.setAttribute("role","button"),e.setAttribute("tabindex","0"),e.setAttribute("aria-label","Toggle navigation"),t&&e.setAttribute("aria-controls",t.id);var i=function(){e.setAttribute("aria-expanded",a.classList.contains("mobile-expanded")?"true":"false")};e.addEventListener("click",function(){requestAnimationFrame(i)}),e.addEventListener("keydown",function(r){r.key!=="Enter"&&r.key!==" "||(r.preventDefault(),e.click(),requestAnimationFrame(i))}),i()}}document.readyState==="loading"?document.addEventListener("DOMContentLoaded",n):n(),new MutationObserver(n).observe(document.documentElement,{childList:!0,subtree:!0})})();
