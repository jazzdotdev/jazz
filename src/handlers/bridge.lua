
-- Patch require to log all executed modules
_G._require = require
function _G.require (module_name)
    if package.loaded[module_name] == nil then
        _log.trace("[running] " .. module_name)
    end
    return _require(module_name)
end

_G.module_timer = os.clock()

local default_searcher = package.searchers[2]

function require_time(name)
    if string.find(name, "valua") == nil then
        package.preload[name] = function(modulename)
            local created_file = io.open("module.lua", "w+")
            local modulepath = string.gsub(modulename, "%.", "/")
            local path = "/"
            local filename = string.gsub(path, "%?", modulepath)
            local file = io.open(filename, "rb")
            modulepath = "lighttouch-base/" .. modulepath
            if file then
                created_file:write("local module_timer = os.clock()")

                -- count lines to check if last line isn't return 
                local line_count = 0;
                local return_line_num = 0;
                for line in io.lines(modulepath .. ".lua") do
                    if line ~= "" then line_count = line_count + 1 end
                    if string.find(line, "return") ~= nil then
                        return_line_num = line_count
                    end
                end

                local return_on_last_line = false;
                if return_line_num == line_count then return_on_last_line = true end
                local last_line_return = ""
                -- rewrite file content
                local line_num = 0
                for line in io.lines(modulepath .. ".lua") do
                    
                    if line ~= "" then line_num = line_num + 1 end
                    if return_on_last_line == false or line_num ~= line_count then
                        created_file:write(line .. "\n\t")
                    else
                        last_line_return = line
                    end
                end

                created_file:write("\n\tlocal elapsed = (os.clock() - module_timer) * 1000")
                created_file:write("\n\rlog.info(\"" .. modulename .. " done in \" .. elapsed .. \" milliseconds\")")
                if return_on_last_line then
                    created_file:write("\n" .. last_line_return)
                end
                created_file:close()
                -- Compile and return the module
                local to_compile = io.open("module.lua", "rb")
                local compiled = assert(load(assert(to_compile:read("*a")), modulepath))
                os.execute("rm module.lua")
                return compiled
            end
        end
        return require(name)
    else
        return default_searcher("lighttouch-base." .. name)
    end
end

package.searchers[2] = require_time


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
