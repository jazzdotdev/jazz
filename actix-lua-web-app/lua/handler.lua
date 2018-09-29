local utils = require "utils.utils"
local luvent = require "utils.Luvent"
local fs = require "utils.fs"


local req = ctx.msg -- get the request
local response -- declare the response
local possibleResponse



local startTime = os.clock()

function string:split(sep)
    local sep, fields = sep or ":", {}
    local pattern = string.format("([^%s]+)", sep)
    self:gsub(pattern, function(c) fields[#fields+1] = c end)
    return fields
end

---- PACKAGES STEP BY STEP

-- list directories in packages path
-- foreach dir create specific path to events.txt, disabled_actions.txt, rules and actions
-- 'trigger' the loaders


local rules = {} -- rules table to store them from all packages
local events = { } -- events table
local packages_path = "lua/packages" -- directory where packages are stored
-- Splitting packages path to easier determine the name of current package later
local packages_path_modules = packages_path:split( "/" )
local packages_path_length = #packages_path_modules
--

for k, v in pairs (fs.directory_list(packages_path)) do
    local package_name = v:split( "/" )[packages_path_length+1] -- split package path in "/" places and get the last word 
    local events_strings = { } -- events names table
    local event_count = 0
    -- read events file
    local events_file = fs.read_file(v .. "events.txt")
    -- put each line into an strings array
    local s = ""
    for i=1, string.len(events_file) do
        if string.sub( events_file, i, i ) ~= '\n' then
            s = s .. string.sub( events_file, i, i )
        else
            table.insert( events_strings, s)
            s = ""
        end
    end
    
    -- count the lines
    
    for _ in pairs(events_strings) do
        event_count = event_count + 1
        print("counting")
    end
    
    -- create events
    
    for i=1, event_count do
        events[events_strings[i]] = luvent.newEvent()
    end
    
    -- read disabled actions
    local disabled_actions = { }
    local disabled_actions_file = fs.read_file(v .. "disabled_actions.txt")
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
    
    local action_files = fs.get_all_files_in(v .. "actions/")
    for _, file_name in ipairs(action_files) do
        local action_require_name = "packages." .. package_name .. ".actions." .. string.sub( file_name, 0, string.len( file_name ) - 4 )
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

    -- rule loader
    for k, v in pairs(fs.directory_list(packages_path)) do
        local package_name = v:split( "/" )[packages_path_length+1] -- split package path in "/" places and get the last word 
        local rule_files = fs.get_all_files_in(v .. "rules/")
        for _, file_name in ipairs(rule_files) do
            local rule_require_name = "packages." .. package_name .. ".rules." .. string.sub(file_name, 0, string.len( file_name ) - 4)
            local rule_require = require(rule_require_name)
            print("[rule loading] " .. file_name)
            table.insert(rules, rule_require)
        end
    end
    ---

end

-- try/catch in case of errors 

utils.try(function()
    events["reqProcess"]:trigger(req) -- try to process request and give response
    for k, v in pairs(rules) do -- rule trigger
        v.rule(req, events)
    end

end, function(err)
    print("[ERROR]")
    print(err)
    response = { 
        status = 500,
         body = '{ "error": ' .. "try-catch error" .. ' }',  -- if something go wrong give error 500 in response
    }
end)
print("detla time" .. os.clock() - startTime)

return response

