(function () {
  function buildBrandLogo() {
    var logo = document.createElement('img');
    logo.className = 'docslime-brand-logo';
    logo.src = '/assets/favicon.svg';
    logo.alt = '';
    logo.decoding = 'async';
    logo.setAttribute('aria-hidden', 'true');
    return logo;
  }

  function enhanceBrandLabel(target) {
    if (!target || target.dataset.docslimeBrandEnhanced === 'true') {
      return;
    }

    var label = target.textContent.trim() || 'DocSlime';
    var wordmark = document.createElement('span');
    wordmark.className = 'docslime-brand-wordmark';
    wordmark.textContent = label;

    target.dataset.docslimeBrandEnhanced = 'true';
    target.textContent = '';
    target.appendChild(buildBrandLogo());
    target.appendChild(wordmark);
  }

  function enhanceBranding() {
    normalizeSidebarHeading();
    enhanceBrandLabel(document.querySelector('.sidebar-header .docslime-sidebar-title > a, .sidebar-header h1 > a'));
  }

  function normalizeSidebarHeading() {
    var heading = document.querySelector('.sidebar-header h1');
    if (!heading) {
      return;
    }

    var title = document.createElement('div');
    title.className = (heading.className ? heading.className + ' ' : '') + 'docslime-sidebar-title';
    if (heading.id) {
      title.id = heading.id;
    }

    while (heading.firstChild) {
      title.appendChild(heading.firstChild);
    }

    heading.parentNode.replaceChild(title, heading);
  }

  function enhanceMobileMenu() {
    var control = document.querySelector('.sidebar-menu-button');
    var sidebar = document.querySelector('.sidebar');
    var nav = document.querySelector('.sidebar-nav');

    if (!control || !sidebar || control.dataset.docslimeEnhanced === 'true') {
      return;
    }

    if (nav && !nav.id) {
      nav.id = 'docslime-sidebar-nav';
    }

    control.dataset.docslimeEnhanced = 'true';
    control.setAttribute('role', 'button');
    control.setAttribute('tabindex', '0');
    control.setAttribute('aria-label', 'Toggle navigation');
    if (nav) {
      control.setAttribute('aria-controls', nav.id);
    }

    var syncExpanded = function () {
      control.setAttribute('aria-expanded', sidebar.classList.contains('mobile-expanded') ? 'true' : 'false');
    };

    control.addEventListener('click', function () {
      requestAnimationFrame(syncExpanded);
    });

    control.addEventListener('keydown', function (event) {
      if (event.key !== 'Enter' && event.key !== ' ') {
        return;
      }

      event.preventDefault();
      control.click();
      requestAnimationFrame(syncExpanded);
    });

    syncExpanded();
  }

  function buildHeroMascot() {
    var mascot = document.createElement('div');
    mascot.className = 'docslime-hero-mascot';
    mascot.setAttribute('aria-hidden', 'true');

    var logo = buildBrandLogo();
    logo.className = '';

    var page = document.createElement('div');
    page.className = 'docslime-hero-page';

    var title = document.createElement('strong');
    title.textContent = 'PRODUCT.md';

    var section = document.createElement('span');
    section.textContent = '# Vision';

    page.appendChild(title);
    page.appendChild(section);
    page.appendChild(document.createElement('i'));
    page.appendChild(document.createElement('i'));
    page.appendChild(document.createElement('i'));

    mascot.appendChild(logo);
    mascot.appendChild(page);
    return mascot;
  }

  function enhanceHeroArt() {
    var side = document.querySelector('body[data-source-file="docs/index.md"] .hero-side');

    if (!side || side.dataset.docslimeHeroEnhanced === 'true') {
      return;
    }

    var pre = side.querySelector('pre');
    if (!pre) {
      return;
    }

    var wrapper = document.createElement('div');
    wrapper.className = 'docslime-hero-art';
    wrapper.setAttribute('aria-label', 'DocSlime repo tree and blue slime mascot');

    pre.classList.add('docslime-hero-tree');
    pre.parentNode.insertBefore(wrapper, pre);
    wrapper.appendChild(pre);
    wrapper.appendChild(buildHeroMascot());

    var badge = document.createElement('p');
    badge.className = 'docslime-hero-badge';
    badge.textContent = 'Structured. Actionable. Agent-ready.';
    wrapper.appendChild(badge);

    side.dataset.docslimeHeroEnhanced = 'true';
  }

  function findNearestHeading(element) {
    var current = element;

    while (current && current !== document.body) {
      var sibling = current.previousElementSibling;
      while (sibling) {
        if (sibling.matches && sibling.matches('h1, h2, h3, h4, h5, h6')) {
          return sibling.textContent.trim();
        }

        var nested = sibling.querySelector && sibling.querySelector('h1, h2, h3, h4, h5, h6');
        if (nested) {
          return nested.textContent.trim();
        }

        sibling = sibling.previousElementSibling;
      }

      current = current.parentElement;
    }

    return '';
  }

  function enhanceScrollableRegions() {
    var regions = document.querySelectorAll('.table-wrapper, pre');

    Array.prototype.forEach.call(regions, function (region) {
      if (region.dataset.docslimeScrollEnhanced === 'true') {
        return;
      }

      if (region.scrollWidth <= region.clientWidth && region.scrollHeight <= region.clientHeight) {
        return;
      }

      if (!region.hasAttribute('tabindex')) {
        region.setAttribute('tabindex', '0');
      }

      if (!region.hasAttribute('role')) {
        region.setAttribute('role', 'region');
      }

      if (!region.hasAttribute('aria-label')) {
        var heading = findNearestHeading(region);
        var fallback = region.matches('pre') ? 'Scrollable code example' : 'Scrollable table';
        region.setAttribute('aria-label', heading ? fallback + ' in ' + heading : fallback);
      }

      region.dataset.docslimeScrollEnhanced = 'true';
    });
  }

  function enhanceDocSlime() {
    enhanceBranding();
    enhanceMobileMenu();
    enhanceHeroArt();
    enhanceScrollableRegions();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', enhanceDocSlime);
  } else {
    enhanceDocSlime();
  }

  new MutationObserver(function () {
    enhanceDocSlime();
  }).observe(document.documentElement, {
    childList: true,
    subtree: true
  });

  var resizeTimer;
  window.addEventListener('resize', function () {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(enhanceScrollableRegions, 120);
  });
})();
