const rust = import('./pkg/index');

let gizmoClient;

let playing = false;

rust.then(async m => {


    let req = await fetch("resources.json");
    let res = await req.json();

    const objectsData = [];
    const objs = res.objects;
    for (let i = 0; i < objs.length; i++) {
        const obj = objs[i];

        req = await fetch(obj.url);
        res = await req.text();
        objectsData.push(res);
    }

    gizmoClient = new m.Client({
        objects_data: objectsData
    });
    setupEvents();
});

const render = () => {

    gizmoClient.update();

    if (playing) {
        window.requestAnimationFrame(render);
    }
}

const setupEvents = () => {

    document.querySelector("#play").addEventListener("click", (e) => {
        playing = true;
        document.querySelector("#play").className = "hidden";
        document.querySelector("#pause").className = "";
        render();
    });
    document.querySelector("#pause").addEventListener("click", (e) => {
        playing = false;
        document.querySelector("#play").className = "";
        document.querySelector("#pause").className = "hidden";
    })
    document.querySelector("#draw").addEventListener("click", (e) => {
        gizmoClient.draw();
    });

    document.querySelector("#clear").addEventListener("click", (e) => {
        gizmoClient.clear(0x00);
    });
}