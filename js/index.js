const pkg = import("../pkg/index.js");

pkg.then(module => {
    let tps = 0;
    let tpsHistory = [];
    let run = true;
    
    setInterval(() => {
        let start = Date.now();
        let end = Date.now();
        while ((end - start) < 1001) {
            const gameState = module.tick();
            end = Date.now();
            tps += 1;
        }
        tpsHistory.push(tps);
        tps = 0;
    }, 1000);

    setInterval(() => {
        let sum = 0;
        tpsHistory.map(x => sum += x);
        console.log(`Average TPS(TicksPerSecond) over 5 seconds is: ${sum / tpsHistory.length}`)
        tpsHistory = [];
    }, 5000);

    // // simulate 60 frames, every frame a tick
    // const tickTimes = 60 * 60; 
    // const ticks = [];
    // let first = true;
    // for (let i = 0; i < tickTimes;i++) {
    //     const now = window.performance.now();
    //     const gameState = module.tick();
    //     const end = window.performance.now();
    //     if (!!first) {
    //         console.log(gameState);
    //         first = false;
    //     }
    //     ticks.push(end - now);
    // }
    // let sum = 0;
    // ticks.map(x => sum += x)

    // console.log(`Average tick speed (ms): ${sum / ticks.length}`)

    // TODO requestAnimationframe or offload work to Workers?
})
// const mem = import("../pkg/index_bg.wasm");

// pkg
//     .then(wasm => {
//         mem
//         .then(module => {
//             const memory = module.memory;
//         })
//         .catch(console.error)
//     })
//     .catch(console.error)