function rule(req, events)
    print("rule respond_to_any_request")
    --if req.method == "GET" and req.path:match("*") then
        print("respond_to_any_request triggered the event")
        events["request_received"]:trigger(req)
    --end
end
return {
    rule = rule
}
