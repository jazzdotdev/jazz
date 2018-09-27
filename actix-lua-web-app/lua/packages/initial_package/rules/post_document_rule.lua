function rule(req, events)
    if req.method == "POST" and req.path == "/" then
        events[5]:trigger(req)
    end
end

return{
    rule = rule
}
