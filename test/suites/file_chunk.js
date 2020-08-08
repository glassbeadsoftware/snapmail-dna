const { conductorConfig } = require('../config')
const { sleep, split_file } = require('../utils')


module.exports = scenario => {

    // scenario("test write/get tiny file", async (s, t) => {
    //     const {alex} = await s.players({alex: conductorConfig}, true)
    //     const fileChunks = split_file("alex")
    //     console.log('fileChunks: ' + JSON.stringify(fileChunks))
    //     const params = {
    //         data_hash: fileChunks.dataHash,
    //         chunk_index: 0,
    //         chunk_total: fileChunks.numChunks,
    //         chunk: fileChunks.chunks[0],
    //     }
    //     const file_address = await alex.call("app", "snapmail", "write_chunk", params)
    //     console.log('file_address: ' + JSON.stringify(file_address))
    //     t.match(file_address.Ok, RegExp('Qm*'))
    //
    //     const paramsList = {data_hash: fileChunks.dataHash}
    //     const resultList = await alex.call("app", "snapmail", "get_chunk_list", paramsList)
    //     console.log('resultList: ' + JSON.stringify(resultList))
    //     t.deepEqual(resultList.Ok.length, 1)
    //
    //     const chunk_address = file_address.Ok
    //     const params2 = {chunk_address}
    //     const result = await alex.call("app", "snapmail", "get_chunk", params2)
    //     console.log('result: ' + JSON.stringify(result))
    //     t.deepEqual(result.Ok, fileChunks.chunks[0])
    // })
    //
    //
    // scenario("test write/get multi-chunk file", async (s, t) => {
    //     const {alex} = await s.players({alex: conductorConfig}, true)
    //     // -- Create huge file as string
    //     // const data_string = create_huge_string(18)
    //     //const data_string = "x".repeat(250)
    //     const data_string = "123465789".repeat(1 * 1024 * 1024 / 10)
    //     // const data_string = "0123465789ABCDEF";
    //     // const data_string = "0123465789ABCDEFGHIJKLMNOPQRS";
    //     console.log('data_string_size : ' + data_string.length)
    //
    //     // split file
    //     const fileChunks = split_file(data_string)
    //     // console.log('fileChunks: ' + JSON.stringify(fileChunks))
    //
    //     // Write chunks
    //     var chunk_list = [];
    //     for (var i = 0; i < fileChunks.numChunks; ++i) {
    //         //console.log('chunk' + i + ': ' + fileChunks.chunks[i])
    //         const chunk_params = {
    //             data_hash: fileChunks.dataHash,
    //             chunk_index: i,
    //             chunk: fileChunks.chunks[i],
    //         }
    //         const chunk_address = await alex.call("app", "snapmail", "write_chunk", chunk_params)
    //         console.log('chunk_address' + i + ': ' + JSON.stringify(chunk_address))
    //         t.match(chunk_address.Ok, RegExp('Qm*'))
    //         chunk_list.push(chunk_address.Ok)
    //     }
    //     chunk_list = chunk_list.reverse();
    //
    //     // Get chunks
    //     let result_string = ''
    //     for (var i = chunk_list.length - 1; i >= 0; --i) {
    //         const params2 = {chunk_address: chunk_list[i]}
    //         const result = await alex.call("app", "snapmail", "get_chunk", params2)
    //         // console.log('get_result' + i + ': ' + JSON.stringify(result))
    //         result_string += result.Ok
    //     }
    //     t.deepEqual(data_string, result_string)
    // })
}