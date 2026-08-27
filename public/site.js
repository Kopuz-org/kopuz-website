/* Progressive enhancement for the Kopuz site: the OS-aware download button,
   the screenshot reveal, the player bar readout and the shelf copy button.
   Everything here re-runs after a client-side navigation, so every step has to
   be safe to repeat and must not write a value that is already in place. */
(function () {
  "use strict";

  var RELEASES = "https://github.com/Kopuz-org/kopuz/releases";
  var WORDS_PER_MINUTE = 200;

  var sections = [];
  var totalSeconds = 0;
  var scrollQueued = false;
  var revealObserver = null;

  function hydrated() {
    return document.documentElement.hasAttribute("data-hydrated");
  }

  // The label is picked by CSS from the boot script's html[data-os], so this
  // only rewrites the target. Attribute writes are safe before hydration;
  // text writes are not.
  function applyOs() {
    if (!document.documentElement.dataset.os) return;
    var ctas = document.querySelectorAll("[data-os-cta]");
    for (var i = 0; i < ctas.length; i += 1) {
      if (ctas[i].getAttribute("href") === RELEASES) continue;
      ctas[i].setAttribute("href", RELEASES);
      ctas[i].setAttribute("target", "_blank");
      ctas[i].setAttribute("rel", "noopener noreferrer");
    }
  }

  applyOs();

  var TILT_OK =
    window.matchMedia &&
    matchMedia("(hover: hover) and (pointer: fine)").matches &&
    matchMedia("(prefers-reduced-motion: no-preference)").matches;

  function tiltFrames() {
    if (!TILT_OK) return;
    var hosts = document.querySelectorAll("[data-tilt]:not([data-tilt-ready])");
    for (var i = 0; i < hosts.length; i += 1) attachTilt(hosts[i]);
  }

  /* Frames that share a stage (the home moments) share one tilt, so the wipe
     between screenshots reads as a single frame. */
  function attachTilt(host) {
    var group = host.closest("[data-tilt-group]") || host;
    var frames = group === host
      ? [host.querySelector(".frame")].filter(Boolean)
      : Array.prototype.slice.call(group.querySelectorAll(".moment-art .frame"));
    if (!frames.length) return;
    host.setAttribute("data-tilt-ready", "");

    var state = group.__tilt;
    if (!state) {
      state = group.__tilt = { targetX: 0, targetY: 0, tiltX: 0, tiltY: 0, raf: 0, frames: frames };
    }
    state.frames = frames;

    var RANGE = 5;
    var DAMP = 0.1;

    function tick() {
      state.tiltX += (state.targetX - state.tiltX) * DAMP;
      state.tiltY += (state.targetY - state.tiltY) * DAMP;
      var settled = Math.abs(state.targetX - state.tiltX) < 0.02 && Math.abs(state.targetY - state.tiltY) < 0.02;
      if (settled) { state.tiltX = state.targetX; state.tiltY = state.targetY; }
      for (var i = 0; i < state.frames.length; i += 1) {
        state.frames[i].style.setProperty("--tilt-x", state.tiltX.toFixed(2) + "deg");
        state.frames[i].style.setProperty("--tilt-y", state.tiltY.toFixed(2) + "deg");
      }
      state.raf = settled ? 0 : requestAnimationFrame(tick);
    }
    function kick() { if (!state.raf) state.raf = requestAnimationFrame(tick); }

    function unit(value) {
      return Math.max(-1, Math.min(1, value));
    }

    /* Every frame in the group shares one viewport rect, so the pointer is
       measured against that rect and not against the card it is over: the
       theming card is four segments taller and would read its own angle. */
    host.addEventListener("pointermove", function (event) {
      var rect = state.frames[0].getBoundingClientRect();
      if (!rect.width || !rect.height) return;
      state.targetY = unit(((event.clientX - rect.left) / rect.width) * 2 - 1) * RANGE;
      state.targetX = -unit(((event.clientY - rect.top) / rect.height) * 2 - 1) * RANGE;
      kick();
    });
    host.addEventListener("pointerleave", function () {
      state.targetX = 0;
      state.targetY = 0;
      kick();
    });
  }

  function revealFrames() {
    var pending = document.querySelectorAll(".reveal:not(.in)");
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

  /* The theming card is taller than the others by one screen per theme. Scrolling
     that extra height wipes the stacked layers open from the start edge, one at a
     time, while the frame itself holds still. */
  var themeStages = [];
  var GROW = 1.15;
  var GROW_OVER = 120;
  var COPY_GAP = 16;

  function scanThemes() {
    themeStages = [];
    var stages = document.querySelectorAll("[data-theme-stage]");
    for (var i = 0; i < stages.length; i += 1) {
      var stage = stages[i];
      var card = stage.closest(".moment");
      var layers = Array.prototype.slice.call(stage.querySelectorAll(".theme-layer"));
      if (!card || layers.length < 2) continue;
      // The frame's layers and the desks around it wipe together, so the index
      // on the element is what ties a desk to the theme it belongs to.
      var parts = Array.prototype.slice.call(stage.querySelectorAll("[data-theme-index]"));
      themeStages.push({
        stage: stage,
        card: card,
        layers: layers,
        parts: parts.map(function (node) {
          return {
            node: node,
            index: Number(node.getAttribute("data-theme-index")),
            desk: node.classList.contains("theme-desk"),
          };
        }),
        frame: stage.querySelector(".theme-frame"),
        copy: card.querySelector(".moment-copy-inner"),
        caption: stage.querySelector("[data-theme-caption]"),
        base: 0,
        width: 0,
        deskWidth: 0,
        left: 0,
        bleed: 0,
        grow: 1,
        growTo: 1,
      });
    }
    measureThemes();
    paintThemes();
  }

  /* Layout sizes, not painted ones: the scene is scaled, so a rect taken from it
     would carry the scale into every pixel value written back into it. */
  function measureThemes() {
    for (var i = 0; i < themeStages.length; i += 1) {
      var item = themeStages[i];
      var peers = item.card.parentNode
        ? item.card.parentNode.querySelectorAll(".moment:not(.moment-themes)")
        : [];
      var base = 0;
      for (var j = 0; j < peers.length && !base; j += 1) {
        base = peers[j].getBoundingClientRect().height;
      }
      // The card scrolls one plain moment's worth of height before the first
      // wipe, so the stage arrives the way the other frames do.
      item.base = base || window.innerHeight * 0.64;

      var desk = item.stage.querySelector(".theme-desk");
      item.width = item.frame ? item.frame.offsetWidth : item.stage.offsetWidth;
      item.deskWidth = desk ? desk.offsetWidth : item.width;
      item.left = item.frame ? item.frame.offsetLeft : 0;
      item.bleed = desk ? item.left - desk.offsetLeft : 0;

      /* Growing about the end edge swings the desk's start edge, not the
         frame's, back towards the copy, so that edge is what the cap measures.
         The end edge is the transform origin, so its rect survives the scale. */
      var reach = item.stage.offsetWidth - (item.left - item.bleed);
      var room = item.copy
        ? item.stage.getBoundingClientRect().right -
          item.copy.getBoundingClientRect().right - COPY_GAP
        : 0;
      item.growTo = reach > 0 ? Math.max(1, Math.min(GROW, room / reach)) : 1;
    }
  }

  function setVar(node, name, value) {
    if (node.style.getPropertyValue(name) !== value) {
      node.style.setProperty(name, value);
    }
  }

  function clamp01(value) {
    return Math.min(1, Math.max(0, value));
  }

  function paintThemes() {
    for (var i = 0; i < themeStages.length; i += 1) paintTheme(themeStages[i]);
  }

  function paintTheme(item) {
    var rect = item.card.getBoundingClientRect();
    var painted = item.frame.getBoundingClientRect();
    // Undo the scale already on the scene: it grows about the vertical centre.
    var frameHeight = painted.height / item.grow;
    var frameTop = painted.top + (painted.height - frameHeight) / 2;

    /* The seam with the previous card has to cross a frame the same size as that
       card's own, so nothing grows until the card's top edge is past it. */
    var growing = clamp01((frameTop - 16 - rect.top) / GROW_OVER);
    var grow = 1 + (item.growTo - 1) * growing;
    setVar(item.stage, "--grow", grow.toFixed(4));
    item.grow = grow;

    setVar(item.stage, "--exit", clamp01((rect.bottom - frameTop) / frameHeight).toFixed(3));

    var travel = rect.height - item.base;
    var start = window.innerHeight / 2 - item.base / 2;
    // Where there is growth, the wipes wait for it to finish.
    if (item.growTo > 1) start = Math.min(start, frameTop - 16 - GROW_OVER);
    var progress = travel > 0 ? clamp01((start - rect.top) / travel) : 0;

    /* The scroll buys one segment per wipe plus one at the front, which the
       grown preview holds on the theme already on screen. */
    var steps = item.layers.length - 1;
    var pos = Math.min(steps, Math.max(0, progress * (steps + 1) - 1));

    /* The sweep is measured across the desk, which is wider than the frame by
       the bleed on both sides. Driving it off the frame's width instead would
       leave the desk's last bleed covered for good. */
    for (var i = 0; i < item.parts.length; i += 1) {
      var part = item.parts[i];
      if (part.index < 1) continue;
      var sweep = clamp01(pos - (part.index - 1));
      var xDesk = sweep * item.deskWidth;
      if (part.desk) {
        setVar(part.node, "--reveal", (sweep * 100).toFixed(2) + "%");
        setVar(part.node, "--desk-reveal", xDesk.toFixed(1) + "px");
      } else {
        var reveal = item.width > 0 ? clamp01((xDesk - item.bleed) / item.width) : 0;
        setVar(part.node, "--reveal", (reveal * 100).toFixed(2) + "%");
      }
    }

    var fraction = pos - Math.floor(pos);
    var moving = pos > 0 && pos < steps && fraction > 0;
    setVar(item.stage, "--line-x", (fraction * item.deskWidth).toFixed(1) + "px");
    setVar(item.stage, "--line-on", moving ? "1" : "0");

    if (item.caption && hydrated()) {
      // The caption names the incoming theme once its wipe is under way, not
      // at the midpoint.
      var settled = item.layers[Math.min(steps, Math.ceil(pos - 0.08))];
      write(item.caption, settled.getAttribute("data-theme-label") || "");
    }
  }

  function readingSeconds() {
    var main = document.querySelector("main");
    if (!main) return 0;
    var words = (main.textContent || "").trim().split(/\s+/).length;
    return Math.max(30, Math.round((words / WORDS_PER_MINUTE) * 60));
  }

  function clock(seconds) {
    var whole = Math.max(0, Math.round(seconds));
    var minutes = Math.floor(whole / 60);
    var rest = whole % 60;
    return minutes + ":" + (rest < 10 ? "0" : "") + rest;
  }

  /* Writing textContent replaces the node's child, which the observer below
     reads as a childList mutation and answers with a full re-scan. During a
     scroll that is one re-scan per frame, which is what starves the paint, so
     an existing text node is edited in place instead. */
  function write(node, text) {
    if (!node || node.textContent === text) return;
    var only = node.childNodes.length === 1 ? node.firstChild : null;
    if (only && only.nodeType === 3) {
      only.nodeValue = text;
    } else {
      node.textContent = text;
    }
  }

  function paintPlayer() {
    var fill = document.querySelector("[data-player-fill]");
    var elapsed = document.querySelector("[data-player-elapsed]");
    var total = document.querySelector("[data-player-total]");
    var current = document.querySelector("[data-player-section]");
    if ((!fill && !current) || !hydrated()) return;

    var doc = document.documentElement;
    var scrollable = doc.scrollHeight - window.innerHeight;
    var fraction = scrollable > 0 ? Math.min(1, Math.max(0, window.scrollY / scrollable)) : 0;

    if (fill) {
      var percent = (fraction * 100).toFixed(2) + "%";
      if (fill.style.getPropertyValue("--p") !== percent) {
        fill.style.setProperty("--p", percent);
      }
    }
    write(elapsed, clock(totalSeconds * fraction));
    write(total, clock(totalSeconds));

    if (current) {
      var title = "";
      for (var i = 0; i < sections.length; i += 1) {
        if (sections[i].getBoundingClientRect().top <= 80) {
          title = sections[i].getAttribute("data-title") || "";
        }
      }
      write(current, title);
    }
  }

  function paintAll() {
    paintPlayer();
    paintJump();
    paintThemes();
  }

  function onScroll() {
    if (scrollQueued) return;
    scrollQueued = true;
    window.requestAnimationFrame(function () {
      scrollQueued = false;
      paintAll();
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

  function scan() {
    sections = Array.prototype.slice.call(document.querySelectorAll("main [data-title]"));
    totalSeconds = readingSeconds();
    applyOs();
    revealFrames();
    tiltFrames();
    scanThemes();
    paintPlayer();
    spyJump();
  }

  if (window.Lenis && matchMedia("(prefers-reduced-motion: no-preference)").matches) {
    new window.Lenis({ autoRaf: true, lerp: 0.1, anchors: true });
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

  /* Every text write waits on hydration, so a position the page arrived at
     without a scroll event of its own leaves the last painted label in place:
     a restored scroll offset, a step back into the bfcache, or the framework's
     own first write over the node. Each of those gets a repaint. */
  window.addEventListener("load", paintAll);
  window.addEventListener("pageshow", paintAll);
  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", function () {
    measureThemes();
    onScroll();
  }, { passive: true });
  window.addEventListener("popstate", scan);

  // Leptos swaps the contents of <main> on a client-side navigation without a
  // document load, so the section list has to be rebuilt from the DOM itself.
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
    // Hydration finishing is what unblocks the text writes above. The repaint
    // waits a frame so it lands after the framework's own first write.
    new MutationObserver(function () {
      scan();
      window.requestAnimationFrame(paintAll);
    }).observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-hydrated"],
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", start);
  } else {
    start();
  }
})();
