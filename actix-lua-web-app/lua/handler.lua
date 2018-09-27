local utils = require "utils.utils"
local luvent = require "utils.Luvent"
local fs = require "utils.fs"
-- local test_client_action = require "actions.test-client"
-- local debug_action = require "actions.debug"
-- local post_document_action = require "actions.post_document"
-- local get_document_action = require "actions.get_document_by_type"
-- local get_documents_action = require "actions.get_documents_by_type"

-- reqProcess = luvent.newEvent() -- create event for request processing
-- reqProcess2 = luvent.newEvent() 
-- reqProcess_document = luvent.newEvent()
-- reqProcess_documents = luvent.newEvent()
-- reqProcess_post_document = luvent.newEvent()

local req = ctx.msg -- get the request
local response -- declare the response
local possibleResponse

local startTime = os.clock()

local event_count = 0
local events = { }
-- read events file
local events_file = fs.read_file("lua/events.txt")

-- put each line into an array
local s = ""
for i=1, string.len(events_file) do
    if string.sub( events_file, i, i ) ~= '\n' then
        s = s .. string.sub( events_file, i, i )
    else
        table.insert( events, s)
        s = ""
    end
end

-- count the lines

for _ in pairs(events) do
    event_count = event_count + 1
    print("counting")
end

-- create events

for i=1, event_count do
    --print(i)
    events[i] = luvent.newEvent()
end

-- read disabled actions
local disabled_actions = { }
local disabled_actions_file = fs.read_file("lua/disabled_actions.txt")
local s = ""
for i=1, string.len(disabled_actions_file) do
    if string.sub( disabled_actions_file, i, i ) ~= '\n' then
        s = s .. string.sub( disabled_actions_file, i, i )
    else
        table.insert( disabled_actions, s )
        s = ""
    end
end
---

function isDisabled(action_file_name)
    for k, v in pairs(disabled_actions) do
        print (v)
        if action_file_name == v then 
            return true 
        end
    end

    return false
end

-- actions loader

local action_files = fs.get_all_files_in("lua/actions/")
for _, file_name in ipairs(action_files) do
    local action_require_name = "actions." .. string.sub( file_name, 0, string.len( file_name ) - 4 )
    print(action_require_name)
    local action_require = require(action_require_name)

    for k, v in pairs(action_require.event) do
        local action = events[v]:addAction(
            function(req)
                possibleResponse = action_require.action(req)
                if possibleResponse ~= nil then
                    if possibleResponse.body ~= nil then
                        response = possibleResponse
                    end
                end
            end
        )
        events[v]:setActionPriority(action, action_require.priority)
        if isDisabled(file_name) then
            events[v]:disableAction(action)
        end
    end
end

-- end of actions loader


-- try/catch in case of errors 

utils.try(function()
    
    events[1]:trigger(req) -- try to process request and give response
    -- rule loader and checker
    local rule_files = fs.get_all_files_in("lua/rules/")
    for _, file_name in ipairs(rule_files) do
        local rule_require_name = "rules." .. string.sub(file_name, 0, string.len( file_name ) - 4)
        local rule_require = require(rule_require_name)
        rule_require.rule(req, events)
    end
    ---

end, function(err)
    print(err)
    response = { 
        status = 500,
         body = '{ "error": ' .. "try-catch error" .. ' }',  -- if something go wrong give error 500 in response
    }
end)

events[2]:trigger(req)
print("detla time" .. os.clock() - startTime)
return response

