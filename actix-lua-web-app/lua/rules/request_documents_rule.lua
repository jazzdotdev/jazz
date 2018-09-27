function rule(req, events)
    if req.method == "GET" and req.path:match("^/%a+/?$" ) then
        events[4]:trigger(req)
    end
end
return{
    rule = rule
}