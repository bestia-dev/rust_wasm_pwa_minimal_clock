if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('service_worker.js').then(function (registration) {
        console.log('Registration succeeded.');
    }).catch(function (error) {
        console.log('Registration failed with ' + error);
    });
};
//Listen for claiming of our ServiceWorker
navigator.serviceWorker.addEventListener('controllerchange', function () {
    console.log('Service worker status changed: ', this.controller.state);
    // Listen for changes in the state of our ServiceWorker
    navigator.serviceWorker.controller.addEventListener('statechange', function () {
        // If the ServiceWorker becomes "activated", let the user know they can go offline!
        if (this.state === 'activated') {
            window.location.reload();
        }
    });
});
/* applicationCache is deprecated
// check if a new cache is available on page load.
window.addEventListener('load', function () {
    window.applicationCache.addEventListener('updateready', function () {
        if (window.applicationCache.status === window.applicationCache.UPDATEREADY) {
            window.applicationCache.swapCache();
            window.location.reload();
        } else {
            // Manifest didn't changed. Nothing new to server.
        }
    }, false);
}, false);
*/