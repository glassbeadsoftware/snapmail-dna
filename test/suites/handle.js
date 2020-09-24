const { conductorConfig } = require('../config')


//
const test_getset_handle = async (s, t) => {
    const {alex, billy} = await s.players({alex: conductorConfig, billy: conductorConfig}, true)
    //console.log(alex)
    const name = "alex"
    const params = { name }
    const handle_address = await alex.call("app", "snapmail", "set_handle", params)
    console.log('handle_address: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))

    await s.consistency()

    //let playerArray = new Array(alex, billy)
    //const succeeded = await s.simpleConsistency("__snapmail", playerArray)

    const result = await alex.call("app", "snapmail", "get_my_handle", {})
    console.log('result1: ' + JSON.stringify(result))
    t.deepEqual(result.Ok, name)

    const agentId = alex.info('app').agentAddress
    const params2 = { agentId }
    const result2 = await alex.call("app", "snapmail", "get_handle", params2)
    t.deepEqual(result2.Ok, name)

    const result3 = await billy.call("app", "snapmail", "get_handle", params2)
    t.deepEqual(result3.Ok, name)

    // -- Ping -- //

    const result4 = await billy.call("app", "snapmail", "ping_agent", params2)
    t.deepEqual(result4.Ok, true)
};


//
const test_handle_list = async (s, t) => {
    const {alex, billy, camille} = await s.players({alex: conductorConfig, billy: conductorConfig, camille: conductorConfig}, true)

    // Set Alex
    let name = "alex"
    let params = { name }
    let handle_address = await alex.call("app", "snapmail", "set_handle", params)
    console.log('handle_address1: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))
    await s.consistency()

    // Set billy
    name = "billy"
    params = { name }
    handle_address = await billy.call("app", "snapmail", "set_handle", params)
    console.log('handle_address2: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))
    await s.consistency()


    let result = await billy.call("app", "snapmail", "get_all_handles", {})
    console.log('handle_list: ' + JSON.stringify(result))
    t.deepEqual(result.Ok.length, 2)

    // Set camille
    name = "camille"
    params = { name }
    handle_address = await camille.call("app", "snapmail", "set_handle", params)
    console.log('handle_address3: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))
    await s.consistency()

    result = await billy.call("app", "snapmail", "get_all_handles", {})
    console.log('handle_list: ' + JSON.stringify(result))
    t.deepEqual(result.Ok.length, 3)

    // Update Billy
    name = "bob"
    params = { name }
    handle_address = await billy.call("app", "snapmail", "set_handle", params)
    console.log('handle_address4: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))
    await s.consistency()

    result = await billy.call("app", "snapmail", "get_all_handles", {})
    console.log('handle_list updated: ' + JSON.stringify(result))
    t.deepEqual(result.Ok.length, 3)
};

// TODO: Currently this fails as Holochain doesnt allow multiple updates of an entry in one call
const test_set_3_handles = async (s, t) => {
    const {alex} = await s.players({alex: conductorConfig}, true)

    const name = "joe"
    const params0 = { name }
    const handle_address0 = await alex.call("app", "snapmail", "set_handle", params0)
    console.log('handle_address0: ' + JSON.stringify(handle_address0))
    t.match(handle_address0.Ok, RegExp('Qm*'))

    const name1 = "alex"
    const name2 = "billy"
    const name3 = "bob"
    const params = { name1, name2, name3 }
    const handle_address = await alex.call("app", "snapmail", "set_three_handles", params)
    console.log('handle_address: ' + JSON.stringify(handle_address))
    t.match(handle_address.Ok, RegExp('Qm*'))

    let result = await alex.call("app", "snapmail", "get_my_handle", {})
    console.log('result1: ' + JSON.stringify(result))
    t.deepEqual(result.Ok, name3)

    // Get history
    let address = handle_address.Ok
    let params42 = { address }
    let history_result = await alex.call("app", "snapmail", "get_my_handle_history", params42)
    console.log('history_result: ' + JSON.stringify(history_result))
    t.deepEqual(history_result.length, 3)
};


// -- Export scenarios -- //

module.exports = scenario => {
    scenario("test get/set handle", test_getset_handle)
    //scenario("test handle list", test_handle_list)
    // FAILING
    // scenario("test set 3 handles", test_set_3_handles)
}