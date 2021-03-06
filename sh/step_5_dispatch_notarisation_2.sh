_NODE_ADDRESS=$(get_node_address_rpc node="1")
_PATH_TO_CLIENT=$(get_path_to_client)


function _main()
{
    local PATH_TO_DEPLOY="$_PATH_TO_DEMO/outputs/notarisation-2.json"

    DEPLOY_HASH=$(cat "$PATH_TO_DEPLOY" | jq '.hash' | sed -e 's/^"//' -e 's/"$//')

    $_PATH_TO_CLIENT send-deploy \
        --input "$PATH_TO_DEPLOY" \
        --node-address "$_NODE_ADDRESS" \
        > /dev/null 2>&1

    log "dispatched signed notarisation 2:"
    log "... deploy path = $PATH_TO_DEPLOY"
    log "... deploy hash = $DEPLOY_HASH"
}

_main
