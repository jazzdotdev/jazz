
-- Patch require to log all executed modules
_G._require = require
function _G.require (module_name)
    if package.loaded[module_name] == nil then
        _log.trace("[running] " .. module_name)
    end
    return _require(module_name)
end

local default_searcher = package.searchers[2]

function require_time(modulename)
    local start_time = os.clock()
    local module =  default_searcher(modulename)
    local elapsed = (os.clock() - start_time) * 1000
    _log.info(modulename .. " done in " .. elapsed .. " milliseconds")
    return module  
end

package.searchers[2] = _G.require_time


xpcall(function ()
    local init_f, err = loadfile(torchbear.init_filename)
    if not init_f then error(err) end

    local handler = init_f()

    if handler then
        torchbear.handler = handler
    end

end, function (msg)
    msg = tostring(msg)
    local trace = debug.traceback(msg, 3)
    _log.error(trace)
end)

if not torchbear.handler then
    _log.debug("No handler specified")
end
