const { conductorConfig } = require('../config')
const { filterMailList } = require('../utils')


async function setup_handles(s, t, alex, billy) {
    // Make sure Billy has a handle entry
    let name = "billy"
    let params = { name }
    let handle_address = await billy.call("app", "snapmail", "set_handle", params)
    console.log('handle_address1: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))
    // Wait for all network activity to settle
    await s.consistency()

    // Make sure Alex has a handle entry
    name = "alex"
    params = { name }
    handle_address = await alex.call("app", "snapmail", "set_handle", params)
    console.log('handle_address2: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))
    // Wait for all network activity to settle
    await s.consistency()

    // -- Make sure handles are set -- //

    let handle_count = 0
    for (let i = 0; handle_count != 2 && i < 10; i++) {
        result = await billy.call("app", "snapmail", "get_all_handles", {})
        console.log('handle_list: ' + JSON.stringify(result))
        handle_count = result.Ok.length
    }
    t.deepEqual(handle_count, 2)
}

const send_pending_test = async (s, t) => {

    const {alex} = await s.players({alex: conductorConfig}, true)
    const {billy} = await s.players({billy: conductorConfig}, true)
    const billyId = billy.info('app').agentAddress
    console.log('billyId: ' + billyId)

    setup_handles(s, t, alex, billy)

    // send_mail() to Billy
    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [billyId],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }

    await billy.kill()

    await s.consistency()

    const send_result = await alex.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result))
    // Should have pendings
    t.deepEqual(send_result.Ok.cc_pendings, {})

    // Wait for all network activity to settle
    await s.consistency()

    await billy.spawn()
    //
    // handle_address = await billy.call("app", "snapmail", "set_handle", params)
    // console.log('handle_address2: ' + JSON.stringify(handle_address))
    // t.match(handle_address.Ok, RegExp('Qm*'))

    await s.consistency()

    const check_result = await billy.call("app", "snapmail", "check_incoming_mail", {})
    console.log('check_result2      : ' + JSON.stringify(check_result))
    t.deepEqual(check_result.Ok.length, 1)
    t.match(check_result.Ok[0], RegExp('Qm*'))

    const arrived_result = await billy.call("app", "snapmail", "get_all_arrived_mail", {})

    console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok[0]))
    t.deepEqual(arrived_result.Ok.length, 1)
    const mail_adr = arrived_result.Ok[0]
    t.match(mail_adr, RegExp('Qm*'))

    const mail_result = await billy.call("app", "snapmail", "get_mail", {"address": mail_adr})
    console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
    const result_obj = mail_result.Ok.mail
    console.log('result_obj : ' + JSON.stringify(result_obj))

    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, result_obj.payload)

    // -- Send pending Ack -- //

    // Make sure Alex has a handle entry
    name = "alex"
    const params2 = { name }
    let handle_address2 = await alex.call("app", "snapmail", "set_handle", params2)
    console.log('handle_address3: ' + JSON.stringify(handle_address2))
    t.match(handle_address.Ok, RegExp('Qm*'))

    await s.consistency()

    const received_result = await alex.call("app", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result1 : ' + JSON.stringify(received_result.Ok))
    t.deepEqual(received_result.Ok.Err.length, 1)
    t.deepEqual(received_result.Ok.Err[0], billy.info('app').agentAddress)

    await s.consistency()
    await alex.kill()
    await s.consistency()

    const ack_result = await billy.call("app", "snapmail", "acknowledge_mail", {"inmail_address": mail_adr})
    console.log('ack_result1 : ' + ack_result.Ok)

    await s.consistency()
    await alex.spawn()
    await s.consistency()

    const check_result2 = await alex.call("app", "snapmail", "check_incoming_ack", {})
    console.log('check_result2      : ' + JSON.stringify(check_result2))
    t.deepEqual(check_result2.Ok.length, 1)
    t.match(check_result2.Ok[0], RegExp('Qm*'))

    const received_result2 = await alex.call("app", "snapmail", "has_mail_been_received", {"outmail_address": send_result.Ok.outmail})
    console.log('received_result2 : ' + JSON.stringify(received_result2.Ok))
    t.deepEqual(received_result2.Ok.Ok, null)

    const ack_result2 = await billy.call("app", "snapmail", "has_ack_been_received", {"inmail_address": mail_adr})
    console.log('ack_result2 : ' + JSON.stringify(ack_result2))
    t.deepEqual(ack_result2.Ok, true)
};

/**
 *
 */
const send_dm_test = async (s, t) => {

    const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)

    // Make a call to a Zome function
    // indicating the function, and passing it an input
    const send_params = {
        subject: "test-outmail",
        payload: "blablabla",
        to: [alex.info('app').agentAddress],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    console.log('send_resulting')
    const send_result = await billy.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result.Ok))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.Ok.to_pendings, {})

    // Wait for all network activity to settle
    await s.consistency()

    const check_result = await alex.call("app", "snapmail", "check_incoming_mail", {})
    console.log('check_result      : ' + JSON.stringify(check_result.Ok))
    t.deepEqual(check_result.Ok, [])

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

const test_get_all_mails = async (s, t) => {

    const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)

    // Send mail DM
    let send_params = {
        subject: "inmail 1",
        payload: "aaaaaaaa",
        to: [alex.info('app').agentAddress],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    const inMail1Payload = send_params.payload;

    let send_result = await billy.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result1: ' + JSON.stringify(send_result.Ok))
    t.deepEqual(send_result.Ok.to_pendings, {})
    await s.consistency()

    // Send mail DM
    send_params = {
        subject: "inmail 2",
        payload: "bbbb",
        to: [alex.info('app').agentAddress],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    send_result = await billy.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result2: ' + JSON.stringify(send_result.Ok))
    t.deepEqual(send_result.Ok.to_pendings, {})
    const inMail2 = send_result.Ok.outmail;
    await s.consistency()

    // Send mail DM
    send_params = {
        subject: "outmail 3",
        payload: "ccccccc",
        to: [billy.info('app').agentAddress],
        cc: [],
        bcc: [],
        manifest_address_list: []
    }
    send_result = await alex.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result3: ' + JSON.stringify(send_result.Ok))
    t.deepEqual(send_result.Ok.to_pendings, {})
    await s.consistency()

    // Get all mails
    let mail_list_result = await alex.call("app", "snapmail", "get_all_mails", {})
    console.log('mail_list_result1 : ' + JSON.stringify(mail_list_result))
    t.deepEqual(mail_list_result.Ok.length, 3)
    t.deepEqual(mail_list_result.Ok[0].mail.payload, send_params.payload)

    mail_list_result = await billy.call("app", "snapmail", "get_all_mails", {})
    console.log('mail_list_result12 : ' + JSON.stringify(mail_list_result))
    t.deepEqual(mail_list_result.Ok.length, 3)
    t.deepEqual(mail_list_result.Ok[0].mail.payload, send_params.payload)
    const outMail3 = mail_list_result.Ok[0].address;
    console.log('outMail3 : ' + outMail3)

    // -- delete outmail --//

    send_result = await billy.call("app", "snapmail", "delete_mail", {address: inMail2})
    console.log('send_result4: ' + JSON.stringify(send_result.Ok))
    t.match(send_result.Ok, RegExp('Qm*'))
    await s.consistency()

    // Get mail should fail
    let mail_result = await billy.call("app", "snapmail", "get_mail", {"address": inMail2})
    console.log('mail_result : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)

    // Get all mails
    mail_list_result = await billy.call("app", "snapmail", "get_all_mails", {})
    console.log('mail_list_result2 : ' + JSON.stringify(mail_list_result))
    let live_mail_list = filterMailList(mail_list_result.Ok);
    t.deepEqual(live_mail_list.length, 2)
    t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)

    // delete same mail twice should fail
    send_result = await billy.call("app", "snapmail", "delete_mail", {address: inMail2})
    console.log('send_result5: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.Err, {Internal: "Entry Could Not Be Found"})

    // Get all mails - Alex should still see 3
    mail_list_result = await alex.call("app", "snapmail", "get_all_mails", {})
    console.log('mail_list_result3 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result.Ok);
    t.deepEqual(live_mail_list.length, 3)
    t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)

    // -- delete inmail --//

    send_result = await billy.call("app", "snapmail", "delete_mail", {address: outMail3})
    console.log('send_result6: ' + JSON.stringify(send_result.Ok))
    t.match(send_result.Ok, RegExp('Qm*'))
    await s.consistency()

    // Get mail should fail
    mail_result = await billy.call("app", "snapmail", "get_mail", {"address": outMail3})
    console.log('mail_result2 : ' + JSON.stringify(mail_result))
    t.deepEqual(mail_result, null)

    // Get all mails
    mail_list_result = await billy.call("app", "snapmail", "get_all_mails", {})
    console.log('mail_list_result4 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result.Ok);
    t.deepEqual(live_mail_list.length, 1)
    t.deepEqual(live_mail_list[0].mail.payload, inMail1Payload)

    // delete same mail twice should fail
    send_result = await billy.call("app", "snapmail", "delete_mail", {address: outMail3})
    console.log('send_result7: ' + JSON.stringify(send_result))
    t.deepEqual(send_result.Err, {Internal: "Entry Could Not Be Found"})

    // Get all mails - Alex should still see 3
    mail_list_result = await alex.call("app", "snapmail", "get_all_mails", {})
    console.log('mail_list_result3 : ' + JSON.stringify(mail_list_result))
    live_mail_list = filterMailList(mail_list_result.Ok);
    t.deepEqual(live_mail_list.length, 3)
    t.deepEqual(live_mail_list[0].mail.payload, send_params.payload)
};

module.exports = scenario => {
scenario("send pending test", send_pending_test)
//    scenario("send via DM test", send_dm_test)
//scenario("get all mails test", test_get_all_mails)
}
