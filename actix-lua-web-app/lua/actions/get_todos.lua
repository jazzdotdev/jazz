local inspect = require "utils.inspect"
local event = {5}
local priority = 1
local function action (req)
        print("test-client action")
        local new_todo = ClientRequest.build()
            :method("GET")
            :uri("http://jsonplaceholder.typicode.com/todos/")
            :headers({ ["content-type"] = "application/json" })
            :send()
            print(inspect(new_todo))
                response = {
                    body = inspect(new_todo)
                }        

                 return response
end

return {
    event = event,
    action = action,
    priority = priority
}