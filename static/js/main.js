"use strict";

/* ==========================================================================
   Sidebar Collapse/Expand Logic
   ========================================================================== */

function initializeSidebar() {
  console.log("Initializing sidebar...");

  const sidebar = document.querySelector(".admin-sidebar");
  const toggleBtn = document.getElementById("sidebar-toggle");
  const overlay = document.getElementById("sidebar-overlay");
  const mobileToggleBtn = document.querySelector(".jb-aside-mobile-toggle");

  console.log("Sidebar elements found:", {
    sidebar: !!sidebar,
    toggleBtn: !!toggleBtn,
    overlay: !!overlay,
    mobileToggleBtn: !!mobileToggleBtn,
  });

  // Ensure sidebar exists before proceeding
  if (!sidebar) {
    console.error("Sidebar element not found!");
    return;
  }

  console.log("Sidebar successfully found, proceeding with initialization...");

  // Load saved sidebar state from localStorage (desktop only)
  const savedState = localStorage.getItem("sidebar-collapsed");
  console.log("Saved sidebar state:", savedState);

  if (savedState === "true" && window.innerWidth >= 1024) {
    sidebar.classList.add("is-collapsed");
    const toggleIcon = toggleBtn?.querySelector(".mdi");
    if (toggleIcon) {
      toggleIcon.classList.remove("mdi-chevron-left");
      toggleIcon.classList.add("mdi-chevron-right");
    }
    console.log("Applied collapsed state from localStorage");
  }

  // Desktop: Toggle collapse
  if (toggleBtn) {
    console.log("Setting up desktop toggle button");
    toggleBtn.addEventListener("click", function () {
      console.log("Toggle button clicked!");
      sidebar.classList.toggle("is-collapsed");
      const isCollapsed = sidebar.classList.contains("is-collapsed");
      console.log("Sidebar is now:", isCollapsed ? "collapsed" : "expanded");

      // Update icon
      const toggleIcon = toggleBtn.querySelector(".mdi");
      if (toggleIcon) {
        if (isCollapsed) {
          toggleIcon.classList.remove("mdi-chevron-left");
          toggleIcon.classList.add("mdi-chevron-right");
        } else {
          toggleIcon.classList.remove("mdi-chevron-right");
          toggleIcon.classList.add("mdi-chevron-left");
        }
      }

      // Save state
      localStorage.setItem("sidebar-collapsed", isCollapsed ? "true" : "false");
      console.log("Saved state to localStorage:", isCollapsed);
    });
    console.log("Desktop toggle button initialized successfully");
  } else {
    console.warn("Toggle button not found in DOM");
  }

  // Mobile: Toggle sidebar visibility
  if (mobileToggleBtn) {
    console.log("Setting up mobile toggle button");
    mobileToggleBtn.addEventListener("click", function () {
      console.log("Mobile toggle clicked!");
      sidebar.classList.toggle("is-mobile-active");
      const isActive = sidebar.classList.contains("is-mobile-active");
      console.log("Mobile sidebar is now:", isActive ? "visible" : "hidden");

      const icon = mobileToggleBtn.querySelector(".mdi");
      if (icon) {
        icon.classList.toggle("mdi-forwardburger");
        icon.classList.toggle("mdi-backburger");
      }
    });
    console.log("Mobile toggle initialized successfully");
  }

  // Mobile: Close sidebar when clicking overlay
  if (overlay) {
    console.log("Setting up overlay click handler");
    overlay.addEventListener("click", function () {
      console.log("Overlay clicked, closing sidebar");
      sidebar.classList.remove("is-mobile-active");
      const icon = mobileToggleBtn?.querySelector(".mdi");
      if (icon) {
        icon.classList.remove("mdi-backburger");
        icon.classList.add("mdi-forwardburger");
      }
    });
  }

  // Close mobile sidebar when clicking a nav link
  const navLinks = document.querySelectorAll(".admin-sidebar .menu-list a");
  console.log("Found nav links:", navLinks.length);

  navLinks.forEach(function (link) {
    link.addEventListener("click", function () {
      console.log("Nav link clicked:", link.getAttribute("hx-get"));
      if (window.innerWidth < 1024) {
        sidebar.classList.remove("is-mobile-active");
        const icon = mobileToggleBtn?.querySelector(".mdi");
        if (icon) {
          icon.classList.remove("mdi-backburger");
          icon.classList.add("mdi-forwardburger");
        }
      }
    });
  });

  // Reset sidebar state on window resize
  window.addEventListener("resize", function () {
    if (window.innerWidth >= 1024) {
      // Desktop: restore saved state
      sidebar.classList.remove("is-mobile-active");
      const savedState = localStorage.getItem("sidebar-collapsed");
      if (savedState === "true") {
        sidebar.classList.add("is-collapsed");
      }
    } else {
      // Mobile: remove collapsed state, hide sidebar
      sidebar.classList.remove("is-collapsed");
      sidebar.classList.remove("is-mobile-active");
    }
  });

  console.log("Sidebar initialization complete!");
}

/* ==========================================================================
   HTMX Active Link Management
   ========================================================================== */

function updateActiveLink(url) {
  console.log("Updating active link for URL:", url);
  const navLinks = document.querySelectorAll(".admin-sidebar .menu-list a");

  navLinks.forEach(function (link) {
    const linkHref =
      link.getAttribute("hx-get") || link.getAttribute("data-href");

    if (linkHref === url) {
      link.classList.add("is-active", "router-link-active");
      console.log("Activated link:", linkHref);
    } else {
      link.classList.remove("is-active", "router-link-active");
    }
  });
}

function initializeActiveLinks() {
  console.log("Initializing active link management...");

  // Listen to HTMX events to update active link after request
  document.body.addEventListener("htmx:afterRequest", function (event) {
    console.log("HTMX request completed", event.detail);
    const target = event.detail.target;
    if (target && target.id === "content") {
      // Get the URL that was requested
      const requestedUrl = event.detail.pathInfo.requestPath;
      console.log("HTMX loaded:", requestedUrl);
      updateActiveLink(requestedUrl);
    }
  });

  // Listen to clicks on nav links to immediately update active state
  const navLinks = document.querySelectorAll(
    ".admin-sidebar .menu-list a[data-nav-link]",
  );
  navLinks.forEach(function (link) {
    link.addEventListener("click", function (e) {
      const targetUrl =
        link.getAttribute("hx-get") || link.getAttribute("data-href");
      console.log("Nav link clicked, setting active:", targetUrl);
      updateActiveLink(targetUrl);
    });
  });

  // Set initial active state based on current page
  const currentPath = window.location.pathname;
  console.log("Initial page path:", currentPath);
  updateActiveLink(currentPath);

  console.log("Active link management initialized!");
}

/* ==========================================================================
   Aside: Submenus Toggle
   ========================================================================== */

function initializeSubmenus() {
  console.log("Initializing submenu toggles...");
  const menuElements = document.getElementsByClassName("menu is-menu-main");

  Array.from(menuElements).forEach(function (el) {
    const dropdownElements = el.getElementsByClassName("has-dropdown-icon");

    Array.from(dropdownElements).forEach(function (elA) {
      elA.addEventListener("click", function (e) {
        const dropdownIcon = e.currentTarget
          .getElementsByClassName("dropdown-icon")[0]
          .getElementsByClassName("mdi")[0];
        e.currentTarget.parentNode.classList.toggle("is-active");
        dropdownIcon?.classList.toggle("mdi-plus");
        dropdownIcon?.classList.toggle("mdi-minus");
      });
    });
  });
  console.log("Submenu toggles initialized!");
}

/* ==========================================================================
   NavBar Menu Mobile Toggle
   ========================================================================== */

function initializeNavbarToggle() {
  console.log("Initializing navbar menu toggle...");
  const toggleElements = document.getElementsByClassName(
    "jb-navbar-menu-toggle",
  );

  Array.from(toggleElements).forEach(function (el) {
    el.addEventListener("click", function (e) {
      const dropdownIcon = e.currentTarget
        .getElementsByClassName("icon")[0]
        .getElementsByClassName("mdi")[0];
      const targetId = e.currentTarget.getAttribute("data-target");
      const target = document.getElementById(targetId);

      if (target) {
        target.classList.toggle("is-active");
      }

      dropdownIcon?.classList.toggle("mdi-dots-vertical");
      dropdownIcon?.classList.toggle("mdi-close");
    });
  });
  console.log("Navbar toggle initialized!");
}

/* ==========================================================================
   Modal: Open & Close
   ========================================================================== */

function initializeModals() {
  console.log("Initializing modals...");

  // Modal: open
  const modalTriggers = document.getElementsByClassName("jb-modal");
  Array.from(modalTriggers).forEach(function (el) {
    el.addEventListener("click", function (e) {
      const modalTarget = e.currentTarget.getAttribute("data-target");
      const modal = document.getElementById(modalTarget);
      if (modal) {
        modal.classList.add("is-active");
        document.documentElement.classList.add("is-clipped");
      }
    });
  });

  // Modal: close
  const modalCloseButtons = document.getElementsByClassName("jb-modal-close");
  Array.from(modalCloseButtons).forEach(function (el) {
    el.addEventListener("click", function (e) {
      const modal = e.currentTarget.closest(".modal");
      if (modal) {
        modal.classList.remove("is-active");
        document.documentElement.classList.remove("is-clipped");
      }
    });
  });

  console.log("Modals initialized!");
}

/* ==========================================================================
   Notification Dismiss
   ========================================================================== */

function initializeNotifications() {
  console.log("Initializing notification dismiss...");
  const dismissButtons = document.getElementsByClassName(
    "jb-notification-dismiss",
  );

  Array.from(dismissButtons).forEach(function (el) {
    el.addEventListener("click", function (e) {
      const notification = e.currentTarget.closest(".notification");
      if (notification) {
        notification.classList.add("is-hidden");
      }
    });
  });
  console.log("Notifications initialized!");
}

/* ==========================================================================
   Theme Toggle (Light/Dark Mode)
   ========================================================================== */

function initializeThemeToggle() {
  console.log("Initializing theme toggle...");
  const toggle = document.getElementById("theme-toggler");

  if (!toggle) {
    console.warn("Theme toggle button not found");
    return;
  }

  const current = localStorage.getItem("theme") || "light";
  console.log("Current theme:", current);
  document.documentElement.setAttribute("data-theme", current);

  toggle.addEventListener("click", function () {
    const theme = document.documentElement.getAttribute("data-theme");
    const next = theme === "dark" ? "light" : "dark";
    console.log("Switching theme from", theme, "to", next);
    document.documentElement.setAttribute("data-theme", next);
    localStorage.setItem("theme", next);
  });

  console.log("Theme toggle initialized!");
}

/* ==========================================================================
   Main Initialization
   ========================================================================== */

function initializeAll() {
  console.log("=== Starting Argus Admin Panel Initialization ===");
  console.log("Document ready state:", document.readyState);
  console.log("Window width:", window.innerWidth);

  try {
    initializeSidebar();
    initializeActiveLinks();
    initializeSubmenus();
    initializeNavbarToggle();
    initializeModals();
    initializeNotifications();
    initializeThemeToggle();

    console.log("=== Initialization Complete ===");
  } catch (error) {
    console.error("Error during initialization:", error);
  }
}

// Wait for DOM to be fully loaded
if (document.readyState === "loading") {
  console.log("Waiting for DOM to load...");
  document.addEventListener("DOMContentLoaded", initializeAll);
} else {
  console.log("DOM already loaded, initializing immediately...");
  initializeAll();
}
