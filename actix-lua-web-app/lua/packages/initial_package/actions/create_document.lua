local helpers = require("utils.helpers")

local event = {5}
local priority = 1

-- POST /
local function create_document(req)
    local post_uuid = uuid.v4()
    local file = io.open("content/" .. post_uuid, "w")
    local params = {
        title = req.body.title,
        type = req.body.type,
    }

    local yaml_string = yaml.dump(params)
    local document_text = yaml_string .. "\n\n" .. req.body.text
    local document_params = helpers.split_document(document_text, post_uuid)

    file:write(document_text)
    file:close()

    return {
        headers = {
            ["content-type"] = "application/json",
            ["X-Request-ID"] = post_uuid 
        },
        body = render("document.json", { document = document_params }),
    }
end

return {
    create_document = create_document,
    action = create_document,
    event = event,
    priority = priority
}
