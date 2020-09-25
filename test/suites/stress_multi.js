const { conductorConfig } = require('../config')
const { sleep, split_file } = require('../utils')


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test stress 10 agents", test_stress_10_agents)
    //scenario("test stress 30 agents", test_stress_30_agents)

    // CRASH TESTS
    //scenario("test stress 100 agents", test_stress_100_agents)
}

const canBomb = true;
const canAllSend = true;
const canAllAttach = true;


// -- Scenarios -- //

const test_stress_100_agents = async (s, t) => {
    await test_stress_multi(s, t, 100)
}

const test_stress_30_agents = async (s, t) => {
    await test_stress_multi(s, t, 30)
}

const test_stress_10_agents = async (s, t) => {
    await test_stress_multi(s, t, 30)
}

/**
 *
 */
const test_stress_multi = async (s, t, count) => {

    let test_start = Date.now()

    // -- Spawn players -- //

    // Generate list of player names
    let configObj = {}
    for (let i = 0; i < count; i++) {
        let name = 'player' + i
        configObj[name] = conductorConfig
    }

    // Spawn all players
    let spawn_start = Date.now()
    let allPlayers = await s.players(configObj, true)
    let spawn_end = Date.now();
    let spawn_duration = (spawn_end - spawn_start) / 1000

    // Create Map of AgentAddress -> PlayerName
    let playerMap = new Map()
    for (let playerName in allPlayers) {
        if (!Object.prototype.hasOwnProperty.call(allPlayers, playerName)) {
            continue;
        }
        const playa = allPlayers[playerName];
        //console.log({playa})
        const info = playa.instance('app')
        //console.log({info})
        playerMap.set(playerName, info.agentAddress)
    }
    console.log({playerMap})
    const player0 = allPlayers['player0'];
    const player2 = allPlayers['player' + count / 2];
    const player3 = allPlayers['player' + count / 3];
    const allAddresses =[ ...playerMap.values() ];

    // -- Set Handles -- //

    let handles_start = Date.now()

    let handlePromiseArray = new Array(count)
    for (const [playerName, agentAddress] of playerMap) {
        const playa = allPlayers[playerName];
        let params = { name: playerName }
        console.log('** set_handle(): ' + playerName)
        let handle_promise = await playa.call("app", "snapmail", "set_handle", params)
        handlePromiseArray.push(handle_promise)
        //console.log('handle_address: ' + JSON.stringify(handle_address))
        //t.match(handle_address.Ok, RegExp('Qm*'))
    }
    await s.consistency()

    // Make sure handles are set (try 10 times)
    let handle_count = 0

    for (let i = 0; handle_count != count && i < 10; i++) {
        result = await player0.call("app", "snapmail", "get_all_handles", {})
        //console.log('handle_list: ' + JSON.stringify(result))
        handle_count = result.Ok.length
    }
    t.deepEqual(handle_count, count)

    // Done
    let handles_end = Date.now();
    let handles_duration = (handles_end - handles_start) / 1000


    // -- Send BOMB: one Mail to All -- //

    let bomb_start = Date.now();

    if (canBomb) {
        const send_bomb_params = {
            subject: "MsgBomb",
            payload: "BOOM BOOM BOOM",
            to: allAddresses,
            cc: [],
            bcc: [],
            manifest_address_list: []
        }

        console.log('** CALLING: send_mail() - BOMB')
        const send_result = await player0.call("app", "snapmail", "send_mail", send_bomb_params)
        console.log('send_result: ' + JSON.stringify(send_result))
        // Should have no pendings
        t.deepEqual(send_result.Ok.cc_pendings, {})

        await s.consistency()

        const arrived_result = await player2.call("app", "snapmail", "get_all_arrived_mail", {})
        console.log('arrived_result : ' + JSON.stringify(arrived_result.Ok[0]))
        t.deepEqual(arrived_result.Ok.length, 1)
        const mail_adr = arrived_result.Ok[0]
        t.match(mail_adr, RegExp('Qm*'))

        const mail_result = await player2.call("app", "snapmail", "get_mail", {"address": mail_adr})
        //console.log('mail_result : ' + JSON.stringify(mail_result.Ok))
        const result_obj = mail_result.Ok.mail
        //console.log('result_obj : ' + JSON.stringify(result_obj))
        t.deepEqual(send_bomb_params.payload, result_obj.payload)
    }
    // Done
    let bomb_end = Date.now();
    let bomb_duration = (bomb_end - bomb_start) / 1000


    // -- All sends one message -- //

    let all_send_start = Date.now();

    if (canAllSend) {
        let prevAgent = allAddresses[count - 1];
        let prevName = 'player' + (count - 1)
        for (const [playerName, agentAddress] of playerMap) {
            const playa = allPlayers[playerName];
            const send_params = {
                subject: "msg from " + playerName,
                payload: "hello to " + prevName,
                to: [prevAgent],
                cc: [],
                bcc: [],
                manifest_address_list: []
            }

            console.log('** CALLING: send_mail() - ' + playerName)
            const send_result2 = await playa.call("app", "snapmail", "send_mail", send_params)
            //console.log('send_result: ' + JSON.stringify(send_result2))
            // Should have no pendings
            t.deepEqual(send_result2.Ok.cc_pendings, {})
            prevAgent = agentAddress
            prevName = playerName
        }

        await s.consistency()

        const arrived_result2 = await player2.call("app", "snapmail", "get_all_arrived_mail", {})
        console.log('arrived_result2 : ' + JSON.stringify(arrived_result2.Ok[0]))
        t.deepEqual(arrived_result2.Ok.length, 2)
        const mail_adr2 = arrived_result2.Ok[0]
        t.match(mail_adr2, RegExp('Qm*'))

        const mail_result2 = await player2.call("app", "snapmail", "get_mail", {"address": mail_adr2})
        console.log('mail_result2 : ' + JSON.stringify(mail_result2.Ok))
        const result_obj2 = mail_result2.Ok.mail
        console.log('result_obj2 : ' + JSON.stringify(result_obj2))
        t.deepEqual(result_obj2.payload, 'hello to player' + (count / 2))
    }
    // Done
    let all_send_end = Date.now();
    let all_send_duration = (all_send_end - all_send_start) / 1000


    // -- All sends one attachment -- //

    let all_attach_start = Date.now();

    if (canAllAttach) {
        prevAgent = allAddresses[count - 1];
        prevName = 'player' + (count - 1)
        for (const [playerName, agentAddress] of playerMap) {
            const playa = allPlayers[playerName];

            // Create file
            const data_string = playerName.repeat(250 * 1024 / 10)
            const fileChunks = split_file(data_string)

            // Commit chunks
            let chunk_list = [];
            for (let i = 0; i < fileChunks.numChunks; ++i) {
                const chunk_params = {
                    data_hash: fileChunks.dataHash,
                    chunk_index: i,
                    chunk: fileChunks.chunks[i],
                }
                const chunk_address = await playa.call("app", "snapmail", "write_chunk", chunk_params)
                //console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
                t.match(chunk_address.Ok, RegExp('Qm*'))
                chunk_list.push(chunk_address.Ok)
            }
            chunk_list = chunk_list.reverse();

            // Commit manifest
            const manifest_params = {
                data_hash: fileChunks.dataHash,
                filename: "" + playerName + ".str",
                filetype: "str",
                orig_filesize: data_string.length,
                chunks: chunk_list,
            }
            let manifest_address = await playa.call("app", "snapmail", "write_manifest", manifest_params)
            //console.log('manifest_address: ' + JSON.stringify(manifest_address))
            t.match(manifest_address.Ok, RegExp('Qm*'))

            // -- Send Mail
            const send_params = {
                subject: "parcel from " + playerName,
                payload: "payload to " + prevName,
                to: [prevAgent],
                cc: [],
                bcc: [],
                manifest_address_list: [manifest_address.Ok]
            }

            console.log('** CALLING: send_mail() - ' + playerName)
            const send_result2 = await playa.call("app", "snapmail", "send_mail", send_params)
            //console.log('send_result: ' + JSON.stringify(send_result2))
            // Should have no pendings
            t.deepEqual(send_result2.Ok.cc_pendings, {})
            prevAgent = agentAddress
            prevName = playerName
        }

        await s.consistency()

        const arrived_result3 = await player2.call("app", "snapmail", "get_all_arrived_mail", {})
        console.log('arrived_result3 : ' + JSON.stringify(arrived_result3.Ok[0]))
        t.deepEqual(arrived_result3.Ok.length, 3)
        const mail_adr3 = arrived_result3.Ok[0]
        t.match(mail_adr3, RegExp('Qm*'))

        const mail_result3 = await player2.call("app", "snapmail", "get_mail", {"address": mail_adr3})
        console.log('mail_result3 : ' + JSON.stringify(mail_result3.Ok))
        const mail = mail_result3.Ok.mail
        console.log('mail : ' + JSON.stringify(mail))
        t.deepEqual(mail.payload, 'payload to player' + (count / 2))
        // check for equality of the actual and expected results
        t.true(mail.attachments[0].orig_filesize > 200 * 1024)

        // -- Get Attachment
        manifest_address = mail.attachments[0].manifest_address
        // Get chunk list via manifest
        const get_manifest_params = {manifest_address}
        const resultGet = await player2.call("app", "snapmail", "get_manifest", get_manifest_params)
        console.log('get_manifest_result: ' + JSON.stringify(resultGet))
        t.deepEqual(resultGet.Ok.orig_filesize, mail.attachments[0].orig_filesize)
    }

    // Done
    let all_attach_end = Date.now();
    let all_attach_duration = (all_attach_end - all_attach_start) / 1000


    // -- Stats -- //

    let test_end = Date.now();
    let test_duration = (test_end - test_start) / 1000

    console.log("==================================== " + count);
    console.log("Spawn duration      : " + spawn_duration + ' sec')
    console.log("Handles duration    : " + handles_duration + ' sec')
    console.log("Bomb duration       : " + bomb_duration + ' sec')
    console.log("All send duration   : " + all_send_duration + ' sec')
    console.log("All attach duration : " + all_attach_duration + ' sec')
    console.log("------------------------------------");
    console.log("Test duration       : " + test_duration + ' sec')
    console.log("====================================" + count);
}
