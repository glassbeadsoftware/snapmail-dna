const { conductorConfig } = require('../config')

function create_huge_string(power_of_2) {
    const iterations = power_of_2 - 3;
    var x = "12345678";
    for (var i = 0; i < iterations; i++) {
        x = x+x;
    }
    return x;
}
module.exports = scenario => {
    scenario("test write/get tiny file", async (s, t) => {
        const {alex} = await s.players({alex: conductorConfig}, true)
        const data_string = "alex"
        const params = {data_string}
        const file_address = await alex.call("app", "snapmail", "write_file", params)
        console.log('file_address: ' + JSON.stringify(file_address))
        t.match(file_address.Ok, RegExp('Qm*'))

        const address = file_address.Ok
        const params2 = {address}
        const result = await alex.call("app", "snapmail", "get_file", params2)
        console.log('result: ' + JSON.stringify(result))
        t.deepEqual(result, data_string)
    })

    scenario("test write/get 500 KiB file", async (s, t) => {
        const {alex} = await s.players({alex: conductorConfig}, true)
        // const data_string = create_huge_string(18)
        //const data_string = "x".repeat(250)
        // const data_string = "123465789".repeat(25)
        const data_string = "0123465789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        console.log('data_string_size : ' + data_string.length)
        const params = {data_string}
        const file_address = await alex.call("app", "snapmail", "write_file", params)
        console.log('file_address: ' + JSON.stringify(file_address))
        t.match(file_address.Ok, RegExp('Qm*'))

        const address = file_address.Ok
        const params2 = {address}
        const result = await alex.call("app", "snapmail", "get_file", params2)
        //console.log('result: ' + JSON.stringify(result))
        t.deepEqual(result, data_string)
    })
}