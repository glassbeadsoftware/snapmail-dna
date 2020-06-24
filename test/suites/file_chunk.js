const { conductorConfig } = require('../config')
const { sleep } = require('../utils')

const sjcl = require('sjcl')

function create_huge_string(power_of_2) {
    const iterations = power_of_2 - 3;
    var x = "12345678";
    for (var i = 0; i < iterations; i++) {
        x = x+x;
    }
    return x;
}

function sha256(message) {
    // // encode as UTF-8
    // const msgBuffer = new TextEncoder('utf-8').encode(message);
    // // hash the message
    // const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
    // // convert ArrayBuffer to Array
    // const hashArray = Array.from(new Uint8Array(hashBuffer));
    // // convert bytes to hex string
    // const hashHex = hashArray.map(b => ('00' + b.toString(16)).slice(-2)).join('');

    console.log('message: ' + message)
    const myBitArray = sjcl.hash.sha256.hash(message)
    console.log('myBitArray: ' + JSON.stringify(myBitArray))
    const hashHex = sjcl.codec.hex.fromBits(myBitArray)
    console.log('hashHex: ' + hashHex)
    return hashHex;
}

function chunkSubstr(str, size) {
    var numChunks = Math.ceil(str.length / size);
    var chunks = new Array(numChunks);

    for(var i = 0, o = 0; i < numChunks; ++i, o += size) {
        chunks[i] = str.substr(o, size);
    }

    return chunks;
}

const CHUNK_MAX_SIZE = 10;

function split_file(full_data_string) {
    const hash = sha256(full_data_string);
    console.log('hash: ' + hash)
    const chunks = chunkSubstr(full_data_string, CHUNK_MAX_SIZE);

    return {
        dataHash: hash,
        numChunks: chunks.length,
        chunks: chunks,
    }
}

module.exports = scenario => {
    // scenario("test write/get tiny file", async (s, t) => {
    //     const {alex} = await s.players({alex: conductorConfig}, true)
    //     const fileChunks = split_file("alex")
    //     console.log('fileChunks: ' + JSON.stringify(fileChunks))
    //     const params = {
    //         data_hash: fileChunks.dataHash,
    //         chunk_total: fileChunks.numChunks,
    //         first_chunk: fileChunks.chunks[0],
    //     }
    //     const file_address = await alex.call("app", "snapmail", "write_initial_chunk", params)
    //     console.log('file_address: ' + JSON.stringify(file_address))
    //     t.match(file_address.Ok, RegExp('Qm*'))
    //
    //     const initial_address = file_address.Ok
    //     const params2 = {initial_address, index: 0}
    //     const result = await alex.call("app", "snapmail", "get_file", params2)
    //     console.log('result: ' + JSON.stringify(result))
    //     t.deepEqual(result.Ok, fileChunks.chunks[0])
    // })

    scenario("test write/get multi-chunk file", async (s, t) => {
        const {alex} = await s.players({alex: conductorConfig}, true)
        // -- Create huge file as string
        // const data_string = create_huge_string(18)
        //const data_string = "x".repeat(250)
        // const data_string = "123465789".repeat(25)
        const data_string = "0123465789ABCDEF";
        //const data_string = "0123465789ABCDEFGHIJKLMNOPQRS";
        console.log('data_string_size : ' + data_string.length)

        // split file
        const fileChunks = split_file(data_string)
        console.log('fileChunks: ' + JSON.stringify(fileChunks))

        // Write initial chunk
        const params = {
            data_hash: fileChunks.dataHash,
            chunk_total: fileChunks.numChunks,
            first_chunk: fileChunks.chunks[0],
        }
        const file_address = await alex.call("app", "snapmail", "write_initial_chunk", params)
        console.log('file_address: ' + JSON.stringify(file_address))
        t.match(file_address.Ok, RegExp('Qm*'))
        const initial_address = file_address.Ok

        // Write subsequent chunks
        for (var i = 1; i < fileChunks.numChunks; ++i) {
            sleep(10000)
            await s.consistency()
            console.log('chunk' + i + ': ' + fileChunks.chunks[i])
            const chunk_params = {
                data_hash: fileChunks.dataHash,
                chunk: fileChunks.chunks[i],
                initial_address,
            }
            const chunk_address = await alex.call("app", "snapmail", "write_chunk", chunk_params)
            console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
            t.match(chunk_address.Ok, RegExp('Qm*'))
        }

        // Get file
        let result_string = ''
        for (var i = 0; i < fileChunks.numChunks; ++i) {
            await s.consistency()
            sleep(10000)
            const params2 = {initial_address, index: i}
            const result = await alex.call("app", "snapmail", "get_file", params2)
            console.log('get_result' + i + ': ' + JSON.stringify(result))
            result_string += result.Ok
        }
        t.deepEqual(data_string, result_string)
    })
}