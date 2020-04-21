/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require('path')

const { Orchestrator, Config, combine, singleConductor, localOnly, tapeExecutor } = require('@holochain/tryorama')

function sleep(milliseconds) {
  const date = Date.now();
  let currentDate = null;
  do {
    currentDate = Date.now();
  } while (currentDate - date < milliseconds);
}

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/snapmail-dna.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,

    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor,
  ),
})

const dna = Config.dna(dnaPath, 'scaffold-test')
const conductorConfig = Config.gen(
    {myInstanceName: dna}
    ,
    {
        logger: Config.logger({ type: "debug" }),
        // network: {
        //     type: 'sim2h',
        //     sim2h_url: 'ws://sim2h.harris-braun.com:9047'
        // }
    }
    )

orchestrator.registerScenario("test get/set handle", async (s, t) => {
  const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)
  const name = "alex"
  const params = { name }
  const handle_address = await alex.call("myInstanceName", "snapmail", "set_handle", params)
  console.log('handle_address: ' + JSON.stringify(handle_address))
  t.match(handle_address.Ok, RegExp('Qm*'))

  // Wait for all network activity to settle
  await s.consistency()

  const result = await alex.call("myInstanceName", "snapmail", "get_my_handle", {})
  t.deepEqual(result.Ok, name)

  const agentId = alex.info('myInstanceName').agentAddress
  const params2 = { agentId }
  const result2 = await alex.call("myInstanceName", "snapmail", "get_handle", params2)
  t.deepEqual(result2.Ok, name)

  const result3 = await billy.call("myInstanceName", "snapmail", "get_handle", params2)
  t.deepEqual(result3.Ok, name)

})


//
// orchestrator.registerScenario("send pending test", async (s, t) => {
//
//     const {alex} = await s.players({alex: conductorConfig}, true)
//     const {billy} = await s.players({billy: conductorConfig}, true)
//
//     // You can also kill them...
//
//
//     // Make a call to a Zome function
//     // indicating the function, and passing it an input
//     const send_params = {
//         subject: "test-outmail",
//         payload: "blablabla",
//         to: ["42"],
//         cc: [],
//         bcc: []
//     }
//
//     const check_result1 = await billy.call("myInstanceName", "snapmail", "check_incoming_mail", {})
//     console.log('check_result1      : ' + JSON.stringify(check_result1.Ok))
//     t.deepEqual(check_result1.Ok, [])
//
//     await billy.kill()
//
//     const send_result = await alex.call("myInstanceName", "snapmail", "send_mail", send_params)
//     console.log('send_result: ' + JSON.stringify(send_result))
//     // Should receive via DM, so no pendings
//     t.deepEqual(send_result.Ok.to_pendings, {})
//
//     // Wait for all network activity to settle
//     await s.consistency()
//
//     await billy.spawn()
//
//     const check_result = await billy.call("myInstanceName", "snapmail", "check_incoming_mail", {})
//     console.log('check_result2      : ' + JSON.stringify(check_result.Ok))
//     t.deepEqual(check_result.Ok, [])
//
//     const arrived_result = await billy.call("myInstanceName", "snapmail", "get_all_arrived_mail", {})
//
//     console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok[0]))
//     t.deepEqual(arrived_result.Ok.length, 1)
//     const mail_adr = arrived_result.Ok[0]
//
//     const mail_result = await billy.call("myInstanceName", "snapmail", "get_mail", {"address": mail_adr})
//     console.log('mail_result : ' + mail_result.Ok)
//     const result_obj = mail_result.Ok.mail
//     console.log('result_obj : ' + JSON.stringify(result_obj))
//
//     // check for equality of the actual and expected results
//     t.deepEqual(send_params.payload, result_obj.payload)
// })

//
// orchestrator.registerScenario("send via DM test", async (s, t) => {
//
//   const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)
//
//   // Make a call to a Zome function
//   // indicating the function, and passing it an input
//   const send_params = {
//       subject: "test-outmail",
//       payload: "blablabla",
//       to: [alex.info('myInstanceName').agentAddress],
//       cc: [],
//       bcc: []
//   }
//
//   const send_result = await billy.call("myInstanceName", "snapmail", "send_mail", send_params)
//   console.log('send_result: ' + JSON.stringify(send_result.Ok))
//   // Should receive via DM, so no pendings
//   t.deepEqual(send_result.Ok.to_pendings, {})
//
//   // Wait for all network activity to settle
//   await s.consistency()
//
//   const check_result = await alex.call("myInstanceName", "snapmail", "check_incoming_mail", {})
//   console.log('check_result      : ' + JSON.stringify(check_result.Ok))
//   t.deepEqual(check_result.Ok, [])
//
//   const arrived_result = await alex.call("myInstanceName", "snapmail", "get_all_arrived_mail", {})
//
//   console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok[0]))
//   t.deepEqual(arrived_result.Ok.length, 1)
//   const mail_adr = arrived_result.Ok[0]
//
//   const mail_result = await alex.call("myInstanceName", "snapmail", "get_mail", {"address": mail_adr})
//   console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
//   const result_obj = mail_result.Ok.mail
//
//   // check for equality of the actual and expected results
//   t.deepEqual(send_params.payload, result_obj.payload)
//
//   // -- ACK -- //
//
//   const received_result = await billy.call("myInstanceName", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
//   console.log('received_result1 : ' + JSON.stringify(received_result.Ok))
//     t.deepEqual(received_result.Ok.Err.length, 1)
//     t.deepEqual(received_result.Ok.Err[0], alex.info('myInstanceName').agentAddress)
//
//   const ack_result = await alex.call("myInstanceName", "snapmail", "acknowledge_mail", {"inmail_address": mail_adr})
//   console.log('ack_result1 : ' + ack_result.Ok)
//
//   await s.consistency()
//
//   const received_result2 = await billy.call("myInstanceName", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
//   console.log('received_result2 : ' + JSON.stringify(received_result2.Ok))
//   t.deepEqual(received_result2.Ok.Ok, null)
//
//   const ack_result2 = await alex.call("myInstanceName", "snapmail", "has_ack_been_received", {"inmail_address": mail_adr})
//   console.log('ack_result2 : ' + JSON.stringify(ack_result2))
//   t.deepEqual(ack_result2.Ok, true)
// })

orchestrator.run()
