const rp = require('request-promise')
let ENV_URL = "http://127.0.0.1:7746/env/"
let SHOW_URL_STUB = "http://127.0.0.1:7746/rev/show "

/*
If you call this function with no arguments then the env variables are fetched without any "show" data.

if you call this function with one argument "all", then all env variables are fetched with "show" data.

If you include one or more arguments, then the env and show data is fetched for the end vars whose names
match one of the supplied arguments.

None matching argument names are ignored.

Examples:

node spinup.js 
	-- fetches all of the env vars and their addresses.
node spinup.js all
	-- fetches all of the env vars and shows their addresses and "show" data.
node spinup.js account pubkey
	-- fetches the addresses and "show" data of 'account' and 'pubkey'.

*/

function make_env_call(){
    return rp({
        url : ENV_URL,
        method : 'GET',
        json : false
    })
}

function make_show_call(addr){
    let url_str = SHOW_URL_STUB;
    url_str += addr;
    return rp({
        url : url_str,
        method : 'GET',
        json : false
    })
}

async function processURLs(show_all, addr_names){
    let result;
    let env_list  = [];
    result = await make_env_call();
    let results = result.split('\n');
    for (line of results) {
        let env_array = line.split('=');
        if (env_array.length > 1) {
            if (show_all == true || addr_names.includes(env_array[0])) {
                let show_res = await make_show_call(env_array[1]);
                env_array.push( show_res );
                env_list.push( env_array );
            } else if (show_all == false && addr_names.length == 0) {
                env_list.push( env_array );
            }
        }
    }
    return env_list;
}

async function doTask(){
    let show_all = false;
    let addr_names = [];
    args = process.argv
    if (args.length == 3 && args[2] == "all") {
        show_all = true;
    } else if (args.length > 2) {
        addr_names = args.slice(2);
    }

    let result = await processURLs(show_all, addr_names);
    console.log(result);
}

doTask();
