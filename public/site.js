(function () {
  "use strict";

  var WORDS_PER_MINUTE = 200;

  var sections = [];
  var totalSeconds = 0;
  var scrollQueued = false;
  var revealObserver = null;

  function hydrated() {
    return document.documentElement.hasAttribute("data-hydrated");
  }


  var TILT_OK =
    window.matchMedia &&
    matchMedia("(hover: hover) and (pointer: fine)").matches &&
    matchMedia("(prefers-reduced-motion: no-preference)").matches;

  function tiltFrames() {
    if (!TILT_OK) return;
    var hosts = document.querySelectorAll("[data-tilt]:not([data-tilt-ready])");
    for (var i = 0; i < hosts.length; i += 1) attachTilt(hosts[i]);
  }

  // One tilt per stage, so a wipe between screenshots reads as a single frame.
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

    // Measured against the group's frame, not the card under the pointer: the
    // theming card is four segments taller and would read its own angle.
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
    var pending = document.querySelectorAll(".reveal:not(.in), .reveal-sec:not(.in), .reveal-card:not(.in), .reveal-self:not(.in)");
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

  // The theming card carries one extra screen per theme. Scrolling that height
  // wipes the stacked layers open while the frame itself holds still.
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
      // data-theme-index is what ties a desk to the theme it belongs to.
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
        box: card.querySelector(".moment-stage"),
        copy: card.querySelector(".moment-copy-inner"),
        plain: false,
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

  // Layout sizes, not painted ones: a rect off the scaled scene would carry the
  // scale into every value written back into it.
  function measureThemes() {
    for (var i = 0; i < themeStages.length; i += 1) {
      var item = themeStages[i];
      // A viewport too short to pin the frame gets no wipes at all, and CSS
      // owns that call: an unpinned stage is the cue to hold the first theme.
      item.plain = !!item.box && getComputedStyle(item.box).position === "static";
      if (item.plain) continue;

      var peers = item.card.parentNode
        ? item.card.parentNode.querySelectorAll(".moment:not(.moment-themes)")
        : [];
      var base = 0;
      for (var j = 0; j < peers.length && !base; j += 1) {
        base = peers[j].getBoundingClientRect().height;
      }
      item.base = base || window.innerHeight * 0.64;

      var desk = item.stage.querySelector(".theme-desk");
      item.width = item.frame ? item.frame.offsetWidth : item.stage.offsetWidth;
      item.deskWidth = desk ? desk.offsetWidth : item.width;
      item.left = item.frame ? item.frame.offsetLeft : 0;
      item.bleed = desk ? item.left - desk.offsetLeft : 0;

      // The end edge is the transform origin, so its rect survives the scale.
      // Growth swings the desk's start edge, which is what the cap measures.
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

  function holdTheme(item) {
    item.grow = 1;
    setVar(item.stage, "--grow", "1");
    setVar(item.stage, "--exit", "1");
    setVar(item.stage, "--line-on", "0");
    for (var i = 0; i < item.parts.length; i += 1) {
      var part = item.parts[i];
      if (part.index < 1) continue;
      setVar(part.node, "--reveal", "0%");
      if (part.desk) setVar(part.node, "--desk-reveal", "0px");
    }
    if (item.caption && hydrated()) {
      write(item.caption, item.layers[0].getAttribute("data-theme-label") || "");
    }
  }

  function paintTheme(item) {
    if (item.plain) return holdTheme(item);

    var rect = item.card.getBoundingClientRect();
    var painted = item.frame.getBoundingClientRect();
    // Undo the scale already on the scene: it grows about the vertical centre.
    var frameHeight = painted.height / item.grow;
    var frameTop = painted.top + (painted.height - frameHeight) / 2;

    // Nothing grows until the card's top edge clears the seam above it.
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

    // One segment per wipe, plus one at the front held on the theme on screen.
    var steps = item.layers.length - 1;
    var pos = Math.min(steps, Math.max(0, progress * (steps + 1) - 1));

    // Swept across the desk, wider than the frame by the bleed on both sides.
    // Off the frame's width the desk's last bleed would stay covered for good.
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

  // textContent replaces the child, which the observer below answers with a full
  // re-scan: one per frame during a scroll. Edit the text node in place instead.
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
    var seek = document.querySelector("[data-player-seek]");
    if (seek) {
      var now = String(Math.round(fraction * 100));
      if (seek.getAttribute("aria-valuenow") !== now) seek.setAttribute("aria-valuenow", now);
    }
    write(elapsed, clock(totalSeconds * fraction));
    write(total, clock(totalSeconds));

    var index = currentChapter();
    if (current) {
      write(current, index < 0 ? "" : sections[index].getAttribute("data-title") || "");
    }
    paintQueue(index);
  }

  // ── chapters: the player's seek bar, skip buttons and queue jump between the
  // page's [data-title] sections ──

  var NAV_H = 60;
  var queueOpen = false;
  var chapterKey = "";
  var dragging = false;

  function scrollable() {
    return Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
  }

  function chapterTop(section) {
    return Math.max(0, section.getBoundingClientRect().top + window.scrollY - NAV_H);
  }

  // The last sections are often too short to reach the top, so the end of
  // the page counts as being in the last one.
  function currentChapter() {
    var index = -1;
    for (var i = 0; i < sections.length; i += 1) {
      if (sections[i].getBoundingClientRect().top <= NAV_H + 20) index = i;
    }
    if (sections.length && window.scrollY >= scrollable() - 2) index = sections.length - 1;
    return index;
  }

  function chapterAt(fraction) {
    var y = fraction * scrollable();
    var index = 0;
    for (var i = 0; i < sections.length; i += 1) {
      if (chapterTop(sections[i]) <= y + 1) index = i;
    }
    return index;
  }

  function smooth() {
    return !window.matchMedia || !matchMedia("(prefers-reduced-motion: reduce)").matches;
  }

  function goTo(y, instant) {
    window.scrollTo({ top: y, behavior: instant || !smooth() ? "auto" : "smooth" });
  }

  function goToChapter(index) {
    if (index < 0 || index >= sections.length) return;
    goTo(chapterTop(sections[index]));
  }

  function step(direction) {
    var index = currentChapter();
    if (direction < 0 && index >= 0 && sections[index].getBoundingClientRect().top < NAV_H - 40) {
      goToChapter(index);
    } else {
      goToChapter(Math.max(0, Math.min(sections.length - 1, index + direction)));
    }
  }

  // The body observer rescans on every child-list change, including the ones
  // made here, so an unchanged layout must not touch the DOM.
  function buildChapters() {
    var rail = document.querySelector(".player-rail");
    var list = document.querySelector("[data-player-items]");
    var key = String(hydrated()) + scrollable();
    for (var n = 0; n < sections.length; n += 1) {
      key += "|" + Math.round(chapterTop(sections[n])) + (sections[n].getAttribute("data-title") || "");
    }
    if (key === chapterKey && (!rail || rail.querySelector("b"))) return;
    chapterKey = key;
    if (rail) {
      var old = rail.querySelectorAll("b");
      for (var i = 0; i < old.length; i += 1) rail.removeChild(old[i]);
      for (var j = 1; j < sections.length; j += 1) {
        var tick = document.createElement("b");
        tick.style.setProperty("--at", ((chapterTop(sections[j]) / scrollable()) * 100).toFixed(2) + "%");
        rail.appendChild(tick);
      }
    }
    if (list && hydrated()) {
      list.textContent = "";
      for (var k = 0; k < sections.length; k += 1) {
        var item = document.createElement("li");
        var button = document.createElement("button");
        button.type = "button";
        button.setAttribute("data-chapter", String(k));
        var number = document.createElement("span");
        number.textContent = String(k + 1);
        var name = document.createElement("span");
        name.textContent = sections[k].getAttribute("data-title") || "";
        var time = document.createElement("span");
        time.textContent = clock(totalSeconds * (chapterTop(sections[k]) / scrollable()));
        button.appendChild(number);
        button.appendChild(name);
        button.appendChild(time);
        item.appendChild(button);
        list.appendChild(item);
      }
    }
  }

  function paintQueue(index) {
    if (!queueOpen) return;
    var buttons = document.querySelectorAll("[data-chapter]");
    for (var i = 0; i < buttons.length; i += 1) {
      if (i === index) buttons[i].setAttribute("aria-current", "true");
      else buttons[i].removeAttribute("aria-current");
    }
  }

  function setQueue(open, focusFirst) {
    var panel = document.querySelector("[data-player-list]");
    if (!panel) return;
    queueOpen = open;
    if (open) buildChapters();
    if (focusFirst) panel.setAttribute("data-instant", "");
    else panel.removeAttribute("data-instant");
    panel.hidden = !open;
    var toggles = document.querySelectorAll("[data-player-queue]");
    for (var i = 0; i < toggles.length; i += 1) toggles[i].setAttribute("aria-expanded", open ? "true" : "false");
    if (open) {
      paintQueue(currentChapter());
      var target = panel.querySelector("[aria-current]") || panel.querySelector("button");
      if (focusFirst && target) target.focus();
    }
  }

  function seekFraction(event, track) {
    var box = track.getBoundingClientRect();
    var x = (event.clientX - box.left) / box.width;
    if (document.documentElement.dir === "rtl") x = 1 - x;
    return Math.min(1, Math.max(0, x));
  }

  function hint(track, event) {
    var label = track.querySelector("[data-player-hint]");
    if (!label || !sections.length) return;
    var fraction = seekFraction(event, track);
    var box = track.getBoundingClientRect();
    write(label, sections[chapterAt(fraction)].getAttribute("data-title") || "");
    label.style.setProperty("--hint-x", (event.clientX - box.left).toFixed(0) + "px");
    track.classList.add("is-hinting");
  }

  document.addEventListener("pointerdown", function (event) {
    var track = event.target.closest ? event.target.closest("[data-player-seek]") : null;
    if (!track) return;
    event.preventDefault();
    dragging = true;
    track.setPointerCapture(event.pointerId);
    goTo(seekFraction(event, track) * scrollable(), event.pointerType !== "mouse");
    hint(track, event);
  });

  document.addEventListener("pointermove", function (event) {
    var track = event.target.closest ? event.target.closest("[data-player-seek]") : null;
    if (!track) return;
    if (dragging) goTo(seekFraction(event, track) * scrollable(), true);
    if (event.pointerType === "mouse" || dragging) hint(track, event);
  });

  function endDrag(event) {
    var track = event.target.closest ? event.target.closest("[data-player-seek]") : null;
    dragging = false;
    if (track && event.pointerType !== "mouse") track.classList.remove("is-hinting");
  }
  document.addEventListener("pointerup", endDrag);
  document.addEventListener("pointercancel", endDrag);
  document.addEventListener("pointerout", function (event) {
    var track = event.target.closest ? event.target.closest("[data-player-seek]") : null;
    if (track && !dragging && !track.contains(event.relatedTarget)) track.classList.remove("is-hinting");
  });

  document.addEventListener("keydown", function (event) {
    if (event.key === "Escape" && queueOpen) {
      setQueue(false);
      var toggle = document.querySelector("[data-player-queue]:not([hidden])");
      if (toggle) toggle.focus();
      return;
    }
    var track = event.target.closest ? event.target.closest("[data-player-seek]") : null;
    if (!track) return;
    var keys = { ArrowRight: 1, ArrowUp: 1, PageDown: 1, ArrowLeft: -1, ArrowDown: -1, PageUp: -1 };
    if (event.key in keys) {
      event.preventDefault();
      step(keys[event.key]);
    } else if (event.key === "Home") {
      event.preventDefault();
      goTo(0);
    } else if (event.key === "End") {
      event.preventDefault();
      goTo(scrollable());
    }
  });

  document.addEventListener("click", function (event) {
    if (!event.target.closest) return;
    var opener = event.target.closest("[data-open-download]");
    var dialog = document.querySelector("[data-download-dialog]");
    if (opener && dialog && dialog.showModal) {
      event.preventDefault();
      dialog.showModal();
      dialog.focus();
      return;
    }
    if (dialog && (event.target === dialog || event.target.closest("[data-close-dialog]"))) {
      dialog.close();
      return;
    }
    var stepper = event.target.closest("[data-player-step]");
    if (stepper) {
      step(Number(stepper.getAttribute("data-player-step")));
      return;
    }
    var toggle = event.target.closest("[data-player-queue]");
    if (toggle) {
      setQueue(!queueOpen, event.detail === 0);
      return;
    }
    var chapter = event.target.closest("[data-chapter]");
    if (chapter) {
      goToChapter(Number(chapter.getAttribute("data-chapter")));
      setQueue(false);
      return;
    }
    if (queueOpen && !event.target.closest("[data-player-list]")) setQueue(false);
  });

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
    revealFrames();
    tiltFrames();
    scanThemes();
    buildChapters();
    paintPlayer();
    spyJump();
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

  // Text writes wait on hydration, so anything that arrives without a scroll
  // event (bfcache, a restored offset) needs a repaint of its own.
  window.addEventListener("load", function () {
    buildChapters();
    paintAll();
  });
  window.addEventListener("pageshow", paintAll);
  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", function () {
    measureThemes();
    buildChapters();
    onScroll();
  }, { passive: true });
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
    // The repaint waits a frame so it lands after hydration's own first write.
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
