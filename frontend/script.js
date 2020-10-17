
function setYear(event) {
    target = event.target;

    console.log(event.eventName);
    console.log(event.eventType);

    for (ls of target.labels) {
        ls.innerHTML = target.value;
    }

    if (target.name == "notbefore") {
        otherScroller = document.getElementById("notafter");
        if (otherScroller.value < target.value) {
            otherScroller.value = target.value;
            prepareAndFireEvent(otherScroller, "change");
        }
    }
    else {
        otherScroller = document.getElementById("notbefore");
        if (otherScroller.value > target.value) {
            otherScroller.value = target.value;
            prepareAndFireEvent(otherScroller, "change");
        }
    }
}

function prepareAndFireEvent(sender, name) {
    let event = document.createEvent("HTMLEvents");
    event.initEvent(name, true, true);
    event.eventType = name;
    sender.dispatchEvent(event);
}

function setupFields() {
    document.getElementById("notbefore").value = 1950;
    prepareAndFireEvent(document.getElementById("notbefore"), "change");
    document.getElementById("notafter").value = 2020;
    prepareAndFireEvent(document.getElementById("notafter"), "change");

    document.getElementById("dob").valueAsNumber = new Date();
}

function onReset() {

    let current = document.getElementById("notbefore");
    for (lb of current.labels) {
        lb.innerHTML = "1950";
    }

    current = document.getElementById("notafter");
    for (lb of current.labels) {
        lb.innerHTML = "2020";
    }
}
