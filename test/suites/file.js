const { conductorConfig } = require('../config')

module.exports = scenario => {
    scenario("test write/get file", async (s, t) => {
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
}