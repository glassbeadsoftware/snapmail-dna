
function sleep(milliseconds) {
    const date = Date.now();
    let currentDate = null;
    do {
        currentDate = Date.now();
    } while (currentDate - date < milliseconds);
}

function filterMailList(mail_list) {
    let new_list = [];
    for (let mailItem of mail_list) {
        if (mailItem.state.hasOwnProperty('In')) {
            if (mailItem.state.In === 'Deleted') {
                continue;
            }
        }
        if (mailItem.state.hasOwnProperty('Out')) {
            if (mailItem.state.Out === 'Deleted') {
                continue;
            }
        }
        new_list.push(mailItem);
    }
    return new_list;
}
