

local event = {"request_received"}
local priority = 1

local function action (req)  
  response = {
    headers = {
      ["content-type"] = "text/plain",
    },
    body = "hello"
  }
  
  return response
end

return {
    event = event,
    action = action,
    priority = priority
}