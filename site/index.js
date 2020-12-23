const js = import("./node_modules/divein/divein.js");
const pako = require('pako');

js.then(js => {
    let req = new Request('test.gz');
    fetch(req).then((res)=>{
        if (!res.ok) {
            throw new Error('not found');
        }
        return res.arrayBuffer();
    }).then((ab)=>{
        let result = pako.ungzip(new Uint8Array(ab));
        js.test_simulator(result);
    });
});
