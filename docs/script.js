// tauri-plugin-torchlight docs
// ナビ折りたたみ / モバイルメニュー / 右TOC生成 / スクロール追従 / コピーボタン
;(function () {
    "use strict"

    var LANG_KEY = "torchlight-lang"
    var lang = document.documentElement.getAttribute("data-lang") === "ja" ? "ja" : "en"
    var t = {
        ja: { copy: "コピー", copied: "コピーしました", anchor: "このセクションへのリンク" },
        en: { copy: "Copy", copied: "Copied", anchor: "Link to this section" },
    }[lang]

    // ---------- 言語スイッチャ（/ja ↔ /en） ----------
    // 現在ページの言語を記憶（ルートの振り分けが参照する）。
    try {
        localStorage.setItem(LANG_KEY, lang)
    } catch (e) {}

    // 言語切替リンクは現在のセクション(#hash)を引き継ぎ、選択を記憶する。
    var langLinks = document.querySelectorAll("[data-lang-link]")
    langLinks.forEach(function (a) {
        a.setAttribute("data-href", a.getAttribute("href").split("#")[0])
        a.addEventListener("click", function () {
            try {
                localStorage.setItem(LANG_KEY, a.getAttribute("data-lang"))
            } catch (e) {}
        })
    })

    function syncLangLinks() {
        langLinks.forEach(function (a) {
            a.setAttribute("href", a.getAttribute("data-href") + location.hash)
        })
    }

    syncLangLinks()
    window.addEventListener("hashchange", syncLangLinks)

    // ---------- モバイルメニュー ----------
    var sidebar = document.getElementById("sidebar")
    var backdrop = document.getElementById("backdrop")
    var menuToggle = document.getElementById("menuToggle")

    function closeMenu() {
        if (sidebar) sidebar.classList.remove("open")
        if (backdrop) backdrop.classList.remove("show")
    }

    if (menuToggle) {
        menuToggle.addEventListener("click", function () {
            sidebar.classList.toggle("open")
            backdrop.classList.toggle("show")
        })
    }
    if (backdrop) backdrop.addEventListener("click", closeMenu)

    // ---------- サイドバーのグループ折りたたみ ----------
    document.querySelectorAll(".sidebar .group-title").forEach(function (btn) {
        btn.addEventListener("click", function () {
            btn.parentElement.classList.toggle("collapsed")
        })
    })

    // サイドバーのリンク押下でモバイルメニューを閉じる
    document.querySelectorAll(".sidebar a").forEach(function (a) {
        a.addEventListener("click", closeMenu)
    })

    // ---------- コードブロックの言語タグ + コピーボタン ----------
    document.querySelectorAll("pre").forEach(function (pre) {
        var codeLang = pre.getAttribute("data-lang")
        if (codeLang) {
            var tag = document.createElement("span")
            tag.className = "lang-tag"
            tag.textContent = codeLang
            pre.appendChild(tag)
        }
        var btn = document.createElement("button")
        btn.className = "copy-btn"
        btn.type = "button"
        btn.textContent = t.copy
        btn.addEventListener("click", function () {
            var code = pre.querySelector("code")
            var text = code ? code.textContent : pre.textContent
            navigator.clipboard.writeText(text).then(function () {
                btn.textContent = t.copied
                btn.classList.add("copied")
                setTimeout(function () {
                    btn.textContent = t.copy
                    btn.classList.remove("copied")
                }, 1500)
            })
        })
        pre.appendChild(btn)
    })

    // ---------- 見出しにアンカーリンクを付与 ----------
    document.querySelectorAll(".content h2, .content h3").forEach(function (h) {
        var section = h.closest("section")
        var id = h.id || (section && section.id)
        if (!id) return
        if (!h.id) h.id = id
        var a = document.createElement("a")
        a.className = "anchor-h"
        a.href = "#" + id
        a.textContent = "#"
        a.setAttribute("aria-label", t.anchor)
        h.appendChild(a)
    })

    // ---------- 右TOCを見出しから生成 ----------
    var tocList = document.getElementById("tocList")
    var headings = Array.prototype.slice.call(document.querySelectorAll(".content h2, .content h3"))
    var tocLinks = []
    headings.forEach(function (h) {
        var id = h.id
        if (!id || !tocList) return
        var li = document.createElement("li")
        li.className = h.tagName === "H3" ? "lvl-3" : "lvl-2"
        var a = document.createElement("a")
        a.href = "#" + id
        // アンカー記号(#)を除いたテキスト
        a.textContent = h.firstChild ? h.firstChild.textContent.trim() : h.textContent.trim()
        a.setAttribute("data-target", id)
        li.appendChild(a)
        tocList.appendChild(li)
        tocLinks.push(a)
    })

    // ---------- スクロール追従（サイドバー + 右TOC のアクティブ表示） ----------
    var sidebarLinks = Array.prototype.slice.call(
        document.querySelectorAll(".sidebar a[href^='#']"),
    )

    function setActive(id) {
        tocLinks.forEach(function (a) {
            a.classList.toggle("active", a.getAttribute("data-target") === id)
        })
        sidebarLinks.forEach(function (a) {
            a.classList.toggle("active", a.getAttribute("href") === "#" + id)
        })
        // アクティブなTOC項目を可視域へ（過剰スクロールを避けるため nearest 指定）
        var activeToc = tocList && tocList.querySelector("a.active")
        if (activeToc && activeToc.scrollIntoView) {
            activeToc.scrollIntoView({ block: "nearest" })
        }
    }

    if ("IntersectionObserver" in window && headings.length) {
        var visible = {}
        var observer = new IntersectionObserver(
            function (entries) {
                entries.forEach(function (entry) {
                    visible[entry.target.id] = entry.isIntersecting
                })
                // 表示中の見出しのうち最も上のものをアクティブにする
                for (var i = 0; i < headings.length; i++) {
                    if (visible[headings[i].id]) {
                        setActive(headings[i].id)
                        break
                    }
                }
            },
            { rootMargin: "-70px 0px -70% 0px", threshold: 0 },
        )
        headings.forEach(function (h) {
            observer.observe(h)
        })
    }
})()
