(function () {
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
    document.addEventListener('DOMContentLoaded', enhanceMobileMenu);
  } else {
    enhanceMobileMenu();
  }

  new MutationObserver(enhanceMobileMenu).observe(document.documentElement, {
    childList: true,
    subtree: true
  });
})();
