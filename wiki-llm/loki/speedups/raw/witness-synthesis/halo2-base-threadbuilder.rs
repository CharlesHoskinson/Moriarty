<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="generator" content="docs.rs 0.0.0 (f0b23e4a5bd8f96c85fdca936d0cd760092107f6 2026-10-23 )"><link rel="stylesheet" href="/-/static/vendored.css?0-0-0-f0b23e4a5bd8f96c85fdca936d0cd760092107f6-2026-10-23" media="all" />
        <link rel="stylesheet" href="/-/static/style.css?0-0-0-f0b23e4a5bd8f96c85fdca936d0cd760092107f6-2026-10-23" media="all" />
        <link rel="stylesheet" href="/-/static/font-awesome.css?0-0-0-f0b23e4a5bd8f96c85fdca936d0cd760092107f6-2026-10-23" media="all" />

        <link rel="search" href="/-/static/opensearch.xml" type="application/opensearchdescription+xml" title="Docs.rs" />

        <title>halo2-base 0.5.4 - Docs.rs</title><script nonce="M1K97ZRkucX01p+C3ja65ph9zxCX9Rv/gDT8IlFRd9WGjxUI">(function() {
    function applyTheme(theme) {
        if (theme) {
            document.documentElement.dataset.docsRsTheme = theme;
        }
    }

    window.addEventListener("storage", ev => {
        if (ev.key === "rustdoc-theme") {
            applyTheme(ev.newValue);
        }
    });

    // see ./storage-change-detection.html for details
    window.addEventListener("message", ev => {
        if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") {
            applyTheme(ev.data.storage.value);
        }
    });

    applyTheme(window.localStorage.getItem("rustdoc-theme"));
})();</script><script defer type="text/javascript" nonce="M1K97ZRkucX01p+C3ja65ph9zxCX9Rv/gDT8IlFRd9WGjxUI" src="/-/static/menu.js?0-0-0-f0b23e4a5bd8f96c85fdca936d0cd760092107f6-2026-10-23"></script>
        <script defer type="text/javascript" nonce="M1K97ZRkucX01p+C3ja65ph9zxCX9Rv/gDT8IlFRd9WGjxUI" src="/-/static/index.js?0-0-0-f0b23e4a5bd8f96c85fdca936d0cd760092107f6-2026-10-23"></script>
    </head>

    <body class="">
<div class="nav-container">
    <div class="container">
        <div class="pure-menu pure-menu-horizontal" role="navigation" aria-label="Main navigation">
            <form action="/releases/search"
                  method="GET"
                  id="nav-search-form"
                  class="landing-search-form-nav  ">

                
                <a href="/" class="pure-menu-heading pure-menu-link docsrs-logo" aria-label="Docs.rs">
                    <span title="Docs.rs"><span class="fa fa-solid fa-cubes " aria-hidden="true"></span></span>
                    <span class="title">Docs.rs</span>
                </a><ul class="pure-menu-list">
    <script id="crate-metadata" type="application/json">
        
        {
            "name": "halo2-base",
            "version": "0.5.4"
        }
    </script><li class="pure-menu-item">
            <a href="/crate/halo2-base/0.5.4" class="pure-menu-link crate-name" title="Embedded domain specific language (eDSL) for writing circuits with the [`halo2`](https://github.com/axiom-crypto/halo2) API. It simplifies circuit programming to declaring constraints over a single advice and selector column and provides built-in circuit tuning and support for multi-threaded witness generation.">
                <span class="fa fa-solid fa-cube " aria-hidden="true"></span>
                <span class="title">halo2-base-0.5.4</span>
            </a>
        </li><li class="pure-menu-item pure-menu-has-children">
                <a href="#" class="pure-menu-link" aria-label="Platform">
                    <span class="fa fa-solid fa-gears " aria-hidden="true"></span>
                    <span class="title">Platform</span>
                </a>

                
                <ul class="pure-menu-children" id="platforms" data-url="/crate/halo2-base/0.5.4/menus/platforms/"><li class="pure-menu-item">
            <a href="/halo2-base/0.5.4/halo2_base/" class="pure-menu-link" data-fragment="retain" rel="">x86_64-unknown-linux-gnu</a>
        </li></ul>
            </li><li class="pure-menu-item">
                <a href="/crate/halo2-base/0.5.4/features" title="Browse available feature flags of halo2-base-0.5.4" class="pure-menu-link">
                    <span class="fa fa-solid fa-flag " aria-hidden="true"></span>
                    <span class="title">Feature flags</span>
                </a>
            </li>
        
    
    <li id="crate-warnings" class="pure-menu-item hidden" data-url="/-/partial/crate-warnings/halo2-base/"></li>
</ul><div class="spacer"></div>
                
                <div id="abnormalities" class="hidden"></div>

                <ul class="pure-menu-list">
                    <li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="docs.rs">docs.rs</a>
                        <ul class="pure-menu-children aligned-icons"><li class="pure-menu-item"><a class="pure-menu-link" href="/about"><span class="fa fa-solid fa-circle-info " aria-hidden="true"></span> About docs.rs</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/badges"><span class="fa fa-brands fa-fonticons " aria-hidden="true"></span> Badges</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/builds"><span class="fa fa-solid fa-gears " aria-hidden="true"></span> Builds</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/metadata"><span class="fa fa-solid fa-table " aria-hidden="true"></span> Metadata</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/redirections"><span class="fa fa-solid fa-road " aria-hidden="true"></span> Shorthand URLs</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/download"><span class="fa fa-solid fa-download " aria-hidden="true"></span> Download</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/rustdoc-json"><span class="fa fa-solid fa-file-code " aria-hidden="true"></span> Rustdoc JSON</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/releases/queue"><span class="fa fa-solid fa-gears " aria-hidden="true"></span> Build queue</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs" target="_blank"><span class="fa fa-solid fa-shield-halved " aria-hidden="true"></span> Privacy policy</a></li>
                        </ul>
                    </li>
                </ul>
                <ul class="pure-menu-list"><li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="Rust">Rust</a>
                        <ul class="pure-menu-children">
                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://www.rust-lang.org/" target="_blank">Rust website</a></li>
                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/book/" target="_blank">The Book</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/std/" target="_blank">Standard Library API Reference</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/cargo/guide/" target="_blank">The Cargo Guide</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/nightly/clippy" target="_blank">Clippy Documentation</a></li>
                        </ul>
                    </li>
                </ul>
                
                <div id="search-input-nav">
                    <label for="nav-search">
                        <span class="fa fa-solid fa-magnifying-glass " aria-hidden="true"></span>
                    </label>

                    
                    
                    <input id="nav-search" name="query" type="text" aria-label="Find crate by search query" tabindex="-1"
                        placeholder="Find crate"
                        >
                </div>
            </form>
        </div>
    </div>
</div>


<div id="alerts" class="hidden"></div>
    
    <div class="docsrs-package-container">
        <div class="container">
            <div class="description-container">
                

                
                <h1 id="crate-title">
                    halo2-base 0.5.4
                    <span id="clipboard" class="svg-clipboard" title="Copy crate name and version information"></span>
                </h1>

                
                <div class="description">Embedded domain specific language (eDSL) for writing circuits with the [`halo2`](https://github.com/axiom-crypto/halo2) API. It simplifies circuit programming to declaring constraints over a single advice and selector column and provides built-in circuit tuning and support for multi-threaded witness generation.</div>


                <div class="pure-menu pure-menu-horizontal">
                    <ul class="pure-menu-list">
                        
                        <li class="pure-menu-item"><a href="/crate/halo2-base/0.5.4"
                                class="pure-menu-link">
                                <span class="fa fa-solid fa-cube " aria-hidden="true"></span>
                                <span class="title"> Crate</span>
                            </a>
                        </li>

                        
                        <li class="pure-menu-item">
                            <a href="/crate/halo2-base/0.5.4/source/src/virtual_region/copper_chip.rs"
                                class="pure-menu-link pure-menu-active">
                                <span class="fa fa-regular fa-folder-open " aria-hidden="true"></span>
                                <span class="title"> Source</span>
                            </a>
                        </li>

                        
                        <li class="pure-menu-item">
                            <a href="/crate/halo2-base/0.5.4/builds"
                                class="pure-menu-link">
                                <span class="fa fa-solid fa-gears " aria-hidden="true"></span>
                                <span class="title"> Builds</span>
                            </a>
                        </li>

                        
                        <li class="pure-menu-item">
                            <a href="/crate/halo2-base/0.5.4/features"
                               class="pure-menu-link">
                                <span class="fa fa-solid fa-flag " aria-hidden="true"></span>
                                <span class="title">Feature flags</span>
                            </a>
                        </li>
                    </ul>
                </div>
            </div><a href="/halo2-base/0.5.4/halo2_base/" class="doc-link">
                    <span class="fa fa-solid fa-book " aria-hidden="true"></span> Documentation
                </a></div>
    </div>

    <div class="container package-page-container small-bottom-pad">
        <div class="pure-g">
            <div id="side-menu" class="pure-u-1 ">
                <div class="pure-menu package-menu">
                    <ul class="pure-menu-list">
                        
                        
                        <li class="pure-menu-item">
                                <a href="../" class="pure-menu-link"><span class="fa fa-regular fa-folder-open " aria-hidden="true"></span> <span class="text">..</span></a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./lookups/" class="pure-menu-link">
                                    <span class="fa fa-regular fa-folder-open " aria-hidden="true"></span>
                                    <span class="text">lookups</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./tests/" class="pure-menu-link">
                                    <span class="fa fa-regular fa-folder-open " aria-hidden="true"></span>
                                    <span class="text">tests</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./copy_constraints.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>
                                    <span class="text">copy_constraints.rs</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./lookups.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>
                                    <span class="text">lookups.rs</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./manager.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>
                                    <span class="text">manager.rs</span>
                                </a>
                            </li><li class="pure-menu-item">
                                
                                <a href="./mod.rs" class="pure-menu-link">
                                    <span class="fa fa-brands fa-rust " aria-hidden="true"></span>
                                    <span class="text">mod.rs</span>
                                </a>
                            </li></ul>
                </div>
            </div>

            </div>
    </div></body>
</html>