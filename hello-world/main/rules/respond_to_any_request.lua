function rule(req, events)
    if req.method == "GET" and req.path:match("*") then
        events["request_received"]:trigger(req)
    end
end
return {
    rule = rule
}
