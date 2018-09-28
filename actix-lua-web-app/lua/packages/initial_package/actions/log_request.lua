local debug = require "utils.debug"

local event = {"reqProcess"}
local priority = 1


local function log_request(req)
    local log_text = debug.print_req_info_return(req)
    local log_file = io.open("logs/" .. uuid.v4(), "w")

    log_file:write(log_text)
    log_file:close()                                
end

return{
    event = event,
    action = log_request,
    priority = priority
}