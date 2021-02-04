"use strict";

// Incrementing VERSION in CACHE_NAME will kick off the 
// install event and force previously cached
// resources to be cached again.
// but the new service worker will not be activated until all 
//tabs with this webapp are closed.

const CACHE_NAME = '2021.204.1558';

self.addEventListener("install", event => {
    console.log("event install ", CACHE_NAME);
    // the ugly trick of avoiding the waiting phase
    self.skipWaiting();

    event.waitUntil(
        caches.open(CACHE_NAME).then(function (cache) {
            return cache.addAll(
                [
                    "css/rust_wasm_pwa_minimal_clock.css",
                    "pkg/rust_wasm_pwa_minimal_clock_bg.wasm",
                    "pkg/rust_wasm_pwa_minimal_clock.js",
                    "index.html",
                    "start_service_worker.js",
                    "manifest.json",
                    "icons/favicon-32x32.png",
                    "icons/favicon-16x16.png",
                    "icons/favicon-96x96.png",
                    "icons/android-icon-144x144.png",
                    "icons/android-icon-192x192.png",
                    "sound/00oclock.ogg",
                    "sound/01oclock.ogg",
                    "sound/02oclock.ogg",
                    "sound/03oclock.ogg",
                    "sound/04oclock.ogg",
                    "sound/05oclock.ogg",
                    "sound/06oclock.ogg",
                    "sound/07oclock.ogg",
                    "sound/08oclock.ogg",
                    "sound/09oclock.ogg",
                    "sound/10oclock.ogg",
                    "sound/11oclock.ogg",
                    "sound/12oclock.ogg",
                    "sound/13oclock.ogg",
                    "sound/14oclock.ogg",
                    "sound/15oclock.ogg",
                    "sound/16oclock.ogg",
                    "sound/17oclock.ogg",
                    "sound/18oclock.ogg",
                    "sound/19oclock.ogg",
                    "sound/20oclock.ogg",
                    "sound/21oclock.ogg",
                    "sound/22oclock.ogg",
                    "sound/23oclock.ogg"
                ]
            );
        })
    );
});

self.addEventListener("activate", event => {
    console.log("event activate");
    // Delete all caches that aren't CACHE_NAME.
    event.waitUntil(
        caches.keys().then(cacheNames => {
            return Promise.all(
                cacheNames.map(cacheName => {
                    if (CACHE_NAME.indexOf(cacheName) === -1) {
                        // If this cache name isn't right, then delete it.
                        console.log("Deleting out of date cache:", cacheName);
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

self.addEventListener("fetch", event => {
    // console.log("event fetch");
    // Let the browser do its default thing
    // for non-GET requests.
    if (event.request.method != "GET") return;

    // Prevent the default, and handle the request ourselves.
    event.respondWith(async function () {
        // Try to get the response from a cache.
        const cache = await caches.open(CACHE_NAME);
        const cachedResponse = await cache.match(event.request);

        if (cachedResponse) {
            // console.log("from cache");
            // If we found a match in the cache, return it, but also
            // update the entry in the cache in the background.
            event.waitUntil(cache.add(event.request));
            return cachedResponse;
        }

        // If we didn't find a match in the cache, use the network and cache it for later.
        const response = await fetch(event.request);
        cache.put(event.request, response.clone());
        return response;
    }());
});
