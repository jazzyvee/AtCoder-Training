(function () {
    "use strict";

    var API_URL = "https://kenkoooo.com/atcoder/resources/problems.json";
    var CACHE_KEY = "atcoder-problems-cache-v1";
    var CACHE_TTL_MS = 24 * 60 * 60 * 1000; // 1 day

    function getTargets() {
        return Array.prototype.slice.call(document.querySelectorAll("[data-problem-id]"));
    }

    function applyTitles(problems) {
        // Use `name` (plain problem name) instead of `title`/`problem_index`:
        // the API's problem_index is unreliable for some contests (observed
        // off-by-one on abc300), so the letter is derived from our own alias.
        var byId = {};
        for (var i = 0; i < problems.length; i++) {
            byId[problems[i].id] = problems[i].name;
        }
        getTargets().forEach(function (el) {
            var id = el.getAttribute("data-problem-id");
            var name = byId[id];
            if (name) {
                var alias = el.textContent;
                var letter = alias.charAt(0).toUpperCase() + alias.slice(1);
                el.textContent = letter + ". " + name;
                el.removeAttribute("data-problem-id");
            }
        });
    }

    function readCache() {
        try {
            var raw = localStorage.getItem(CACHE_KEY);
            if (!raw) return null;
            var parsed = JSON.parse(raw);
            if (!parsed || !parsed.timestamp || !parsed.problems) return null;
            if (Date.now() - parsed.timestamp > CACHE_TTL_MS) return null;
            return parsed.problems;
        } catch (e) {
            return null;
        }
    }

    function writeCache(problems) {
        try {
            localStorage.setItem(
                CACHE_KEY,
                JSON.stringify({ timestamp: Date.now(), problems: problems })
            );
        } catch (e) {
            // ignore (e.g. storage disabled/full)
        }
    }

    function main() {
        if (getTargets().length === 0) return;

        var cached = readCache();
        if (cached) {
            applyTitles(cached);
            return;
        }

        fetch(API_URL)
            .then(function (res) {
                if (!res.ok) throw new Error("bad response");
                return res.json();
            })
            .then(function (problems) {
                writeCache(problems);
                applyTitles(problems);
            })
            .catch(function () {
                // Network/CORS/API failure: silently keep the alias-only fallback.
            });
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", main);
    } else {
        main();
    }
})();
