const { conductorConfig } = require('../config')
const { split_file } = require('../utils')


module.exports = scenario => {
//
    scenario("test send file async", async (s, t) => {

        // - Create fake file
        const data_string = "0123465789".repeat(500 * 1024 / 10)
        //const data_string = "0123465789"

        const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)
        const billyId = billy.info('app').agentAddress
        console.log('billyId: ' + billyId)

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

        // -- Make sure handles are set -- //

        let handle_count = 0
        for (let i = 0; handle_count != 2 && i < 10; i++) {
            await s.consistency()
            result = await billy.call("app", "snapmail", "get_all_handles", {})
            console.log('handle_listB: ' + JSON.stringify(result))
            handle_count = result.Ok.length
        }
        t.deepEqual(handle_count, 2)
        handle_count = 0
        for (let i = 0; handle_count != 2 && i < 10; i++) {
            await s.consistency()
            result = await alex.call("app", "snapmail", "get_all_handles", {})
            console.log('handle_listA: ' + JSON.stringify(result))
            handle_count = result.Ok.length
        }
        t.deepEqual(handle_count, 2)

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

        // -- Send Mail to Billy
        const send_params = {
            subject: "test-attachment",
            payload: "blablabla",
            to: [billyId],
            cc: [],
            bcc: [],
            manifest_address_list: [manifest_address.Ok],
        }

        await billy.kill();
        await s.consistency();

        const send_result = await alex.call("app", "snapmail", "send_mail", send_params)
        console.log('send_result: ' + JSON.stringify(send_result.Ok))
        // Should receive via DM, so no pendings
        t.deepEqual(send_result.Ok.cc_pendings, {})

        // Wait for all network activity to settle
        await s.consistency()

        await billy.spawn();

        await s.consistency();

        // -- Ping -- //
        const agentId = alex.info('app').agentAddress
        const params2 = { agentId }
        const result4 = await billy.call("app", "snapmail", "ping_agent", params2)
        t.deepEqual(result4.Ok, true)

        let mail_count = 0
        let check_result;
        for (let i = 0; mail_count != 1 && i < 3; i++) {
            await s.consistency()
            check_result = await billy.call("app", "snapmail", "check_incoming_mail", {})
            console.log('' + i + '. check_result2: ' + JSON.stringify(check_result))
            mail_count = check_result.Ok.length
        }
        t.deepEqual(mail_count, 1)
        t.match(check_result.Ok[0], RegExp('Qm*'))
        const mail_adr = check_result.Ok[0]

        // -- Get Mail
        const mail_result = await billy.call("app", "snapmail", "get_mail", {"address": mail_adr})
        console.log('mail_result: ' + JSON.stringify(mail_result.Ok))
        const mail = mail_result.Ok.mail
        // check for equality of the actual and expected results
        t.deepEqual(send_params.payload, mail.payload)
        t.deepEqual(data_string.length, mail.attachments[0].orig_filesize)

        // -- Get Attachment
        manifest_address = mail.attachments[0].manifest_address;

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
        console.log('result_string.length: ' + result_string.length)
        t.deepEqual(data_string.length, result_string.length)
        t.deepEqual(data_string, result_string)
    })
}