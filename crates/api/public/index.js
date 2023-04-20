window.onload = function(event) {
    getComments();
};

/**
 * @param {SubmitEvent} event 
 */
function submitJsonForm(event) {
    event.preventDefault();
    console.log(event);
    console.log('submitJsonForm');
    if (event.target.nodeName !== 'FORM') {
        throw new Error('element is not of type "form"');
    }

    let form = event.target;

    let data = {}
    form.childNodes.forEach(child => {
        if (child.nodeName === 'INPUT') {
            let key = child['name'];
            let value = child['value'];

            data[key] = value;
        }
    });

    let url_path = form.action;

    fetch(url_path, {
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
        method: 'POST',
    });

    location.reload();
}

function getComments() { 
    fetch("/api/comments").then(data => data.json())
    .then(json => {
        let element = document.getElementById("comment-thread");
        element.innerHTML = "";

        json.forEach(comment => {
            element.innerHTML += `<li>${comment.author}: ${comment.body}</li>`
        });
    });
}

console.log('hi');