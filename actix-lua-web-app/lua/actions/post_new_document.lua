local inspect = require "utils.inspect"
local event = {5}
local priority = 1
local function action (req)
        local new_doc = ClientRequest.build()
            :method("POST")
            :uri("http://localhost:3000/test-new-document")
            :headers({ ["content-type"] = "application/json" })
            --:body( req.body )
            :send()
            --print(inspect(new_todo))
                response = {
                    body = inspect(new_doc)
                }        

                 return response
end

return {
    event = event,
    action = action,
    priority = priority
}