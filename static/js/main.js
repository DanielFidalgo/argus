// admin.js
document.addEventListener("alpine:init", () => {
  /* ============================================
     Sidebar Store
     ============================================ */

  Alpine.store("sidebar", {
    collapsed: localStorage.getItem("sidebar-collapsed") === "true",
    mobileOpen: false,

    toggleDesktop() {
      this.collapsed = !this.collapsed;
      localStorage.setItem("sidebar-collapsed", this.collapsed);
    },

    toggleMobile() {
      this.mobileOpen = !this.mobileOpen;
    },

    closeMobile() {
      this.mobileOpen = false;
    },
  });

  /* ============================================
     Theme Store
     ============================================ */

  Alpine.store("theme", {
    current: localStorage.getItem("theme") || "light",

    init() {
      document.documentElement.dataset.theme = this.current;
    },

    toggle() {
      this.current = this.current === "dark" ? "light" : "dark";
      document.documentElement.dataset.theme = this.current;
      localStorage.setItem("theme", this.current);
    },
  });

  /* ============================================
     Active Link Store (HTMX compatible)
     ============================================ */

  Alpine.store("nav", {
    active: window.location.pathname,
    menuOpen: false,

    set(url) {
      this.active = url;
    },

    isActive(url) {
      return this.active === url;
    },
  });

  /* ============================================
     Modal Component
     ============================================ */

  Alpine.data("modal", () => ({
    open: false,

    show() {
      this.open = true;
      document.documentElement.classList.add("is-clipped");
    },

    hide() {
      this.open = false;
      document.documentElement.classList.remove("is-clipped");
    },
  }));

  /* ============================================
     Dropdown Component
     ============================================ */

  Alpine.data("dropdown", () => ({
    open: false,
    toggle() {
      this.open = !this.open;
    },
  }));
});

/* ============================================
   HTMX Integration
   ============================================ */

document.body.addEventListener("htmx:afterRequest", (event) => {
  if (event.detail.target?.id === "content") {
    Alpine.store("nav").set(event.detail.pathInfo.requestPath);
  }
});
