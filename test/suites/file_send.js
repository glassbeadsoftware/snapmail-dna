const { conductorConfig } = require('../config')
const { sleep, split_file } = require('../utils')


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test send file dm tiny", test_send_file_dm_tiny)
    scenario("test send too big file", test_send_file_too_big)

    // LONG TESTS
    //process.env['TRYORAMA_ZOME_CALL_TIMEOUT_MS'] = 90000
    // scenario("test send file dm big", test_send_file_dm_big)
}

// -- Scenarios -- //

const test_send_file_dm_tiny = async (s, t) => {
    await send_file_dm(s, t, 1 * 1024)
}

const test_send_file_dm_big = async (s, t) => {
    await send_file_dm(s, t, 0.9 * 1024 * 1024)
}

/**
 *
 */
async function send_file_dm(s, t, size) {

    const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)

    // - Create fake file
    const data_string = "0123465789".repeat(size / 10)
    // const data_string = "<fake file content>";
    // split file
    const fileChunks = split_file(data_string)
    // Write chunks
    var chunk_list = [];
    for (var i = 0; i < fileChunks.numChunks; ++i) {
        const chunk_params = {
            data_hash: fileChunks.dataHash,
            chunk_index: i,
            chunk: fileChunks.chunks[i],
        }
        const chunk_address = await alex.call("app", "snapmail", "write_chunk", chunk_params)
        console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
        t.match(chunk_address.Ok, RegExp('Qm*'))
        chunk_list.push(chunk_address.Ok)
    }
    chunk_list = chunk_list.reverse();

    // Write manifest
    const manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "fake.str",
        filetype: "str",
        orig_filesize: data_string.length,
        chunks: chunk_list,
    }
    let manifest_address = await alex.call("app", "snapmail", "write_manifest", manifest_params)
    console.log('manifest_address: ' + JSON.stringify(manifest_address))
    t.match(manifest_address.Ok, RegExp('Qm*'))

    // -- Send Mail
    const send_params = {
        subject: "test-attachment",
        payload: "blablabla",
        to: [billy.info('app').agentAddress],
        cc: [],
        bcc: [],
        manifest_address_list: [manifest_address.Ok],
    }

    const send_result = await alex.call("app", "snapmail", "send_mail", send_params)
    console.log('send_result: ' + JSON.stringify(send_result.Ok))
    // Should receive via DM, so no pendings
    t.deepEqual(send_result.Ok.to_pendings, {})

    // Wait for all network activity to settle
    await s.consistency()

    // -- Get Mail
    let new_mail_length = 0;
    let attempt = 0
    let arrived_result = {};
    //while (new_mail_length == 0 && attempt < 10) {
    //     await s.consistency()
    //     sleep(3000)
        attempt += 1;
        arrived_result = await billy.call("app", "snapmail", "get_all_arrived_mail", {})
        console.log('arrived_result : ' + JSON.stringify(arrived_result))
        new_mail_length = arrived_result.Ok.length
    //}
    t.deepEqual(arrived_result.Ok.length, 1)
    const mail_adr = arrived_result.Ok[0]
    const mail_result = await billy.call("app", "snapmail", "get_mail", {"address": mail_adr})
    console.log('mail_result: ' + JSON.stringify(mail_result.Ok))
    const mail = mail_result.Ok.mail
    // check for equality of the actual and expected results
    t.deepEqual(send_params.payload, mail.payload)
    t.deepEqual(data_string.length, mail.attachments[0].orig_filesize)

    // -- Get Attachment
    manifest_address = mail.attachments[0].manifest_address

    // Get chunk list via manifest
    const get_manifest_params = {manifest_address}
    const resultGet = await billy.call("app", "snapmail", "get_manifest", get_manifest_params)
    console.log('get_manifest_result: ' + JSON.stringify(resultGet))
    t.deepEqual(resultGet.Ok.orig_filesize, data_string.length)
    chunk_list = resultGet.Ok.chunks;

    // Get chunks
    let result_string = ''
    for (var i = chunk_list.length - 1; i >= 0; --i) {
        // await s.consistency()
        // sleep(10000)
        const params2 = {chunk_address: chunk_list[i]}
        const result = await billy.call("app", "snapmail", "get_chunk", params2)
        // console.log('get_result' + i + ': ' + JSON.stringify(result))
        result_string += result.Ok
    }
    t.deepEqual(data_string, result_string)
};


/**
 *
 */
const test_send_file_too_big = async (s, t) => {
    const {alex} = await s.players({alex: conductorConfig}, true)

    // - Create fake file
    const data_string = "0123465789";
    // const data_string = "<fake file content>";
    // split file
    const fileChunks = split_file(data_string)
    // Write chunks
    var chunk_list = [];
    for (var i = 0; i < fileChunks.numChunks; ++i) {
        const chunk_params = {
            data_hash: fileChunks.dataHash,
            chunk_index: i,
            chunk: fileChunks.chunks[i],
        }
        const chunk_address = await alex.call("app", "snapmail", "write_chunk", chunk_params)
        console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
        t.match(chunk_address.Ok, RegExp('Qm*'))
        chunk_list.push(chunk_address.Ok)
    }
    chunk_list = chunk_list.reverse();

    // Write manifest
    let manifest_params;
    manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "bigfake.str",
        filetype: "str",
        orig_filesize: 2 * 1024 * 1024,
        chunks: chunk_list,
    }
    let manifest_address = await alex.call("app", "snapmail", "write_manifest", manifest_params)
    console.log('manifest_address: ' + JSON.stringify(manifest_address))
    t.match(JSON.stringify(manifest_address.Err), RegExp('.*ValidationFailed.*'))

    // Empty filesize
    manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "emptyfake.str",
        filetype: "str",
        orig_filesize: 0,
        chunks: chunk_list,
    }
    manifest_address = await alex.call("app", "snapmail", "write_manifest", manifest_params)
    console.log('manifest_address: ' + JSON.stringify(manifest_address))
    t.match(JSON.stringify(manifest_address.Err), RegExp('.*ValidationFailed.*'))

    // emtpy chunk list
    manifest_params = {
        data_hash: fileChunks.dataHash,
        filename: "emptyfake.str",
        filetype: "str",
        orig_filesize: 0.5 * 1024 * 1024,
        chunks: [],
    }
    manifest_address = await alex.call("app", "snapmail", "write_manifest", manifest_params)
    console.log('manifest_address: ' + JSON.stringify(manifest_address))
    t.match(JSON.stringify(manifest_address.Err), RegExp('.*ValidationFailed.*'))
};
