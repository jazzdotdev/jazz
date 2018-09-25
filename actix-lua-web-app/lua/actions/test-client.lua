local inspect = require "utils.inspect"

local function action (req)
    if req.method == "GET" and req.path == "/test-client" then
        print("test-client action")
        local new_todo = ClientRequest.build()
            :method("POST")
            :uri("http://jsonplaceholder.typicode.com/todos/")
            :headers({ ["content-type"] = "application/json" })
            :send()
            print(inspect(new_todo))
                response = {
                    body = inspect(new_todo)
                }        
                 else
                     response = {
                         status = 404,
                     }
                 end

                 return response
end

return {
    action = action
}