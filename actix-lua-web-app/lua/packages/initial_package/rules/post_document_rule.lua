function rule(req, events)
    if req.method == "POST" and req.path == "/" then
        events["reqProcess_post_document"]:trigger(req)
    end
end

return{
    rule = rule
}
