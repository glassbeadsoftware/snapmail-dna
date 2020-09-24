/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

// const path = require('path')

const { Orchestrator, combine, singleConductor, localOnly, tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const networkType = 'sim2h'
const middleware =
    ( networkType === 'websocket'
            ? combine(tapeExecutor(require('tape')), localOnly/*, callSync*/)

            : networkType === 'sim2h'
                ? combine(tapeExecutor(require('tape')), localOnly/*, callSync*/)

                : networkType === 'memory'
                    ? combine(tapeExecutor(require('tape')), localOnly, singleConductor/*, callSync*/)

                    : (() => {throw new Error(`Unsupported memory type: ${networkType}`)})()
    )

const orchestrator = new Orchestrator({
    middleware
    , waiter: {
        softTimeout: 1000,
        hardTimeout: 2000,
        strict: false,
    }
})

require('./suites/handle')(orchestrator.registerScenario)
require('./suites/mail')(orchestrator.registerScenario)
require('./suites/chunk')(orchestrator.registerScenario)
require('./suites/file_send')(orchestrator.registerScenario)
require('./suites/file_send_pending')(orchestrator.registerScenario)
//require('./suites/stress')(orchestrator.registerScenario)

const num = orchestrator.numRegistered()
console.log(`Orchestrator Registered ${num} scenarios`)

var beginning = Date.now();
orchestrator.run().then(stats => {
    let end = Date.now();
    let elapsed = end - beginning;
    console.log(`All ${num} scenarios done. Stats:`)
    console.log(stats)
    console.log("Tests duration: " + elapsed / 1000 + ' sec')
})
