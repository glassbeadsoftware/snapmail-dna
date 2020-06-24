const { conductorConfig } = require('../config')
const { split_file } = require('../utils')


module.exports = scenario => {
    scenario("test write/get multi-chunk file with manifest", async (s, t) => {
        const {alex} = await s.players({alex: conductorConfig}, true)
        // -- Create huge file as string
        // const data_string = create_huge_string(18)
        //const data_string = "x".repeat(250)
        const data_string = "123465789".repeat(1 * 1024 * 1024 / 10)
        // const data_string = "0123465789ABCDEF";
        // const data_string = "0123465789ABCDEFGHIJKLMNOPQRS";
        console.log('data_string_size : ' + data_string.length)

        // split file
        const fileChunks = split_file(data_string)
        // console.log('fileChunks: ' + JSON.stringify(fileChunks))

        // Write chunks
        var chunk_list = [];
        for (var i = 0; i < fileChunks.numChunks; ++i) {
            //console.log('chunk' + i + ': ' + fileChunks.chunks[i])
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
        const manifest_address = await alex.call("app", "snapmail", "write_manifest", manifest_params)
        console.log('manifest_address' + i + ': ' + JSON.stringify(manifest_address))
        t.match(manifest_address.Ok, RegExp('Qm*'))

        // Get chunk list via manifest
        const get_manifest_params = {manifest_address: manifest_address.Ok}
        const resultGet = await alex.call("app", "snapmail", "get_manifest", get_manifest_params)
        console.log('get_manifest_result' + i + ': ' + JSON.stringify(resultGet))
        t.deepEqual(resultGet.Ok.orig_filesize, data_string.length)
        chunk_list = resultGet.Ok.chunks;

        // Get chunks
        let result_string = ''
        for (var i = chunk_list.length - 1; i >= 0; --i) {
            // await s.consistency()
            // sleep(10000)
            const params2 = {chunk_address: chunk_list[i]}
            const result = await alex.call("app", "snapmail", "get_chunk", params2)
            // console.log('get_result' + i + ': ' + JSON.stringify(result))
            result_string += result.Ok
        }
        t.deepEqual(data_string, result_string)
    })
}