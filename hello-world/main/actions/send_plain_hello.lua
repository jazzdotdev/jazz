

local event = {"request_received"}
local priority = 1

print("Hi")

local function action (req)  
  print("send_plain_hello handler")
  response = {
    headers = {
      ["content-type"] = "text/plain",
    },
    body = "hello"
  }
  
  print("response")
  return response
end

return {
    event = event,
    action = action,
    priority = priority
}