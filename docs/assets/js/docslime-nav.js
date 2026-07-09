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
    enhanceBrandLabel(document.querySelector('.sidebar-header h1 > a'));
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

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function () {
      enhanceBranding();
      enhanceMobileMenu();
    });
  } else {
    enhanceBranding();
    enhanceMobileMenu();
  }

  new MutationObserver(function () {
    enhanceBranding();
    enhanceMobileMenu();
  }).observe(document.documentElement, {
    childList: true,
    subtree: true
  });
})();
