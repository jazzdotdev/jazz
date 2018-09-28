function rule(req, events)
    if req.method == "GET" and req.path:match("^/%a+/?$" ) then
        events["reqProcess_documents"]:trigger(req)
    end
end
return{
    rule = rule
}