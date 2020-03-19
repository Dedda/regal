let docReady = function docReady(fn) {
    if (document.readyState === "complete" || document.readyState === "interactive") {
        setTimeout(fn, 1);
    } else {
        document.addEventListener("DOMContentLoaded", fn);
    }
};
let regal = {
    requestJson: function requestJson(url, onSuccess, onError = undefined) {
        let xmlHttp = new XMLHttpRequest();
        xmlHttp.onreadystatechange = function() {
            if (xmlHttp.readyState == 4) {
                if (xmlHttp.status == 200) {
                    onSuccess(JSON.parse(xmlHttp.responseText));
                } else {
                    if (onError !== undefined) {
                        onError(xmlHttp);
                    } else {
                        console.error("Error loading url [" + url + "]: " + xmlHttp.status);
                    }
                }
            }
        };
        xmlHttp.open("GET", url);
        xmlHttp.send(null);
    },
    listInto: function listInto(url, target, builder, onFetched = undefined) {
        regal.requestJson(url, function(data) {
            if (onFetched !== undefined) {
                onFetched(data);
            }
            data.forEach(function(element) {
                target.appendChild(builder(element));
            });
        });
    },
    thumbForPicture: function thumbForPicture(picture) {
        let div = document.createElement("div");
        div.classList.add("thumb-box");
        let a = document.createElement("a");
        a.href = picture.display;
        let img = document.createElement("img");
        img.src = picture.thumb;
        img.classList.add("thumb");
        a.appendChild(img);
        div.appendChild(a);
        return div;
    },
    thumbForGallery: function thumbForGallery(gallery) {
        let div = document.createElement("div");
        div.classList.add("thumb-box");
        let a = document.createElement("a");
        a.href = gallery.display;
        if (gallery.thumb != "none") {
            let img = document.createElement("img");
            img.src = gallery.thumb;
            img.classList.add("thumb");
            let name = document.createElement("p");
            name.innerText = gallery.gallery_name;
            a.appendChild(img);
            a.appendChild(name);
        } else {
            a.innerText = gallery.gallery_name;
        }
        div.appendChild(a);
        return div;
    }
};