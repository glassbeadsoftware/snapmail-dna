const { conductorConfig } = require('../config')
const { sleep, filterMailList } = require('../utils')

/**
 *
 */
const test_stress_1k_mail = async (s, t) => {

    const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)

    // Make a call to a Zome function
    // Indicating the function, and passing it an input
    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [alex.info('app').agentAddress],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    console.log('sending...')
    const send_result = await billy.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result.Ok))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.Ok.to_pendings, {})

    // Wait for all network activity to settle
    await s.consistency()

    const arrived_result = await alex.call("app", "snapmail", "get_all_arrived_mail", {})

    console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok[0]))
    t.deepEqual(arrived_result.Ok.length, 1)
    const mail_adr = arrived_result.Ok[0]

    const mail_result = await alex.call("app", "snapmail", "get_mail", {"address": mail_adr})
    console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
    const result_obj = mail_result.Ok.mail

    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, result_obj.payload)

    // -- ACK -- //

    const received_result = await billy.call("app", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result1 : ' + JSON.stringify(received_result.Ok))
    t.deepEqual(received_result.Ok.Err.length, 1)
    t.deepEqual(received_result.Ok.Err[0], alex.info('app').agentAddress)

    const ack_result = await alex.call("app", "snapmail", "acknowledge_mail", {"inmail_address": mail_adr})
    console.log('ack_result1 : ' + ack_result.Ok)

    await s.consistency()

    const received_result2 = await billy.call("app", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result2 : ' + JSON.stringify(received_result2.Ok))
    t.deepEqual(received_result2.Ok.Ok, null)

    const ack_result2 = await alex.call("app", "snapmail", "has_ack_been_received", {"inmail_address": mail_adr})
    console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    t.deepEqual(ack_result2.Ok, true)
};


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test stress 1k mail", test_stress_1k_mail)

    // LONG TESTS
    // process.env['TRYORAMA_ZOME_CALL_TIMEOUT_MS'] = 90000
    //scenario("test send file async big", test_send_file_async_big)
}