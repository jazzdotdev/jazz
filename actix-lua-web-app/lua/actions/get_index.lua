

if req.method == "GET" and req.path == "/" then
    response = {
        body = render("index.html")
    }
end