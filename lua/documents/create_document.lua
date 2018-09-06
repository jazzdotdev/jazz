local helpers = require("documents.helpers")

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
        },
        body = render("document.json", { document = document_params }),
    }
end

return create_document
