#!/bin/env bash
SELF_PATH=`dirname "$0"`
export TORCHBEAR_LUA_LIBRARY="$SELF_PATH/lua-lib/"
"$SELF_PATH/actix-lua-web/target/debug/actix_lua_web_app" $@