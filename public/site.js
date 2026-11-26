(function () {
  "use strict";

  var scrollQueued = false;
  var revealObserver = null;

  function revealFrames() {
    var pending = document.querySelectorAll(".reveal:not(.in), .reveal-sec:not(.in), .reveal-self:not(.in)");
    if (!pending.length) return;

    if (!("IntersectionObserver" in window)) {
      for (var i = 0; i < pending.length; i += 1) pending[i].classList.add("in");
      return;
    }
    if (!revealObserver) {
      revealObserver = new IntersectionObserver(
        function (entries) {
          entries.forEach(function (entry) {
            if (!entry.isIntersecting) return;
            entry.target.classList.add("in");
            revealObserver.unobserve(entry.target);
          });
        },
        { rootMargin: "0px 0px -10% 0px", threshold: 0.05 }
      );
    }
    for (var j = 0; j < pending.length; j += 1) revealObserver.observe(pending[j]);
  }

  function openDialog(dialog) {
    dialog.showModal();
    dialog.focus();
  }

  document.addEventListener("click", function (event) {
    if (!event.target.closest || event.metaKey || event.ctrlKey || event.shiftKey) return;
    var download = document.querySelector("[data-download-dialog]");
    if (event.target.closest("[data-open-download]") && download && download.showModal) {
      event.preventDefault();
      openDialog(download);
      return;
    }
    var shot = event.target.closest("[data-zoom]");
    var zoom = document.querySelector("[data-zoom-dialog]");
    if (shot && zoom && zoom.showModal) {
      event.preventDefault();
      var thumb = shot.querySelector("img");
      var full = zoom.querySelector("img");
      full.src = shot.href;
      full.alt = thumb ? thumb.alt : "";
      openDialog(zoom);
      return;
    }
    // The zoomed shot closes on any click; the download dialog only on its
    // backdrop or its close button.
    var open = event.target.closest("dialog[open]");
    if (!open) return;
    if (open === zoom || event.target === open || event.target.closest("[data-close-dialog]")) {
      open.close();
    }
  });

  function onScroll() {
    if (scrollQueued) return;
    scrollQueued = true;
    window.requestAnimationFrame(function () {
      scrollQueued = false;
      paintJump();
    });
  }

  var jumpLinks = [], jumpGroups = [];
  function spyJump() {
    var nav = document.querySelector(".feat-jump");
    jumpLinks = nav ? Array.prototype.slice.call(nav.querySelectorAll("a[href*='#']")) : [];
    jumpGroups = Array.prototype.slice.call(document.querySelectorAll(".feat-group[id]"));
    paintJump();
  }

  function paintJump() {
    if (!jumpGroups.length) return;
    var line = window.scrollY + window.innerHeight * 0.3;
    var current = jumpGroups[0];
    for (var i = 0; i < jumpGroups.length; i += 1) {
      if (jumpGroups[i].offsetTop <= line) current = jumpGroups[i];
    }
    if (window.innerHeight + window.scrollY >= document.documentElement.scrollHeight - 2) {
      current = jumpGroups[jumpGroups.length - 1];
    }
    for (var j = 0; j < jumpLinks.length; j += 1) {
      var link = jumpLinks[j];
      var hit = link.getAttribute("href").split("#")[1] === current.id;
      link.classList.toggle("is-current", hit);
      if (hit) link.setAttribute("aria-current", "true"); else link.removeAttribute("aria-current");
    }
  }

  // A spectrum of thin bars rising from behind the hero screenshot, moving
  // like a visualizer. Under reduced motion it draws one still frame.
  var spectrum = null;

  function startSpectrum() {
    var canvas = document.querySelector("[data-spectrum]");
    if (!canvas || (spectrum && spectrum.canvas === canvas)) return;
    var ctx = canvas.getContext("2d");
    if (!ctx) return;

    var still = !window.matchMedia || matchMedia("(prefers-reduced-motion: reduce)").matches;
    var state = { canvas: canvas, visible: true, w: 0, h: 0, fill: null };
    spectrum = state;
    var PITCH = 7;
    var BAR = 2;

    function size() {
      var ratio = Math.min(window.devicePixelRatio || 1, 2);
      state.w = canvas.clientWidth;
      state.h = canvas.clientHeight;
      canvas.width = Math.round(state.w * ratio);
      canvas.height = Math.round(state.h * ratio);
      ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
      // Brightest at the edge the bars stand on, gone at their tips.
      var color = getComputedStyle(canvas).color;
      state.fill = ctx.createLinearGradient(0, state.h, 0, 0);
      state.fill.addColorStop(0, color);
      state.fill.addColorStop(1, "transparent");
      if (still) draw(2);
    }

    function draw(t) {
      var count = Math.floor(state.w / PITCH);
      var offset = (state.w - count * PITCH) / 2;
      ctx.clearRect(0, 0, state.w, state.h);
      ctx.fillStyle = state.fill;
      for (var i = 0; i < count; i += 1) {
        // Three drifting waves stand in for a signal; the bell keeps the
        // tall bars in the middle and lets the ends die out.
        var level =
          0.5 +
          0.25 * Math.sin(i * 0.21 + t * 0.9) +
          0.15 * Math.sin(i * 0.53 - t * 1.4) +
          0.1 * Math.sin(i * 1.37 + t * 2.1);
        var bell = Math.sin((Math.PI * (i + 0.5)) / count);
        var height = state.h * bell * bell * (0.12 + 0.88 * level * level);
        ctx.globalAlpha = 0.35 + 0.65 * level;
        ctx.fillRect(offset + i * PITCH, state.h - height, BAR, height);
      }
    }

    function frame(now) {
      if (spectrum !== state || !canvas.isConnected) {
        if (spectrum === state) spectrum = null;
        return;
      }
      window.requestAnimationFrame(frame);
      if (state.visible) draw(now / 1000);
    }

    size();
    window.addEventListener("resize", size, { passive: true });
    // The bar colour follows the theme toggle.
    new MutationObserver(size).observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });
    if (still) return;
    if ("IntersectionObserver" in window) {
      new IntersectionObserver(function (entries) {
        state.visible = entries[0].isIntersecting;
      }).observe(canvas);
    }
    window.requestAnimationFrame(frame);
  }

  // Leptos re-creates a picker when the release arrives, so each new one is
  // set to the visitor's platform once and then left to the visitor.
  function pickPlatform() {
    var os = document.documentElement.dataset.os;
    if (!os) return;
    document.querySelectorAll("[data-os-picker]:not([data-picked])").forEach(function (picker) {
      picker.setAttribute("data-picked", "");
      var radio = picker.querySelector('input[value="' + os + '"]');
      if (radio) radio.checked = true;
    });
  }

  function scan() {
    pickPlatform();
    revealFrames();
    spyJump();
    startSpectrum();
  }

  document.addEventListener("click", function (event) {
    var button = event.target.closest ? event.target.closest("[data-copy]") : null;
    if (!button || !navigator.clipboard) return;

    navigator.clipboard.writeText(button.getAttribute("data-copy")).then(function () {
      var done = button.getAttribute("data-copied-label") || "Copied";
      var original = button.getAttribute("data-copy-label") || button.textContent;
      button.setAttribute("data-copy-label", original);
      button.textContent = done;
      window.setTimeout(function () {
        button.textContent = button.getAttribute("data-copy-label") || original;
      }, 1400);
    });
  });

  // A page that arrives without a scroll event (bfcache, a restored offset)
  // needs a repaint of its own.
  window.addEventListener("pageshow", paintJump);
  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onScroll, { passive: true });
  window.addEventListener("popstate", scan);

  // Leptos swaps <main> on a client-side navigation without a document load.
  var pendingScan = false;
  var observer = new MutationObserver(function () {
    if (pendingScan) return;
    pendingScan = true;
    window.requestAnimationFrame(function () {
      pendingScan = false;
      scan();
    });
  });

  function start() {
    scan();
    observer.observe(document.body, { childList: true, subtree: true });
  }


  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", start);
  } else {
    start();
  }
})();
