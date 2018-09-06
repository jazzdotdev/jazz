local function split_document(document_text, id)
    local yaml_text, body = document_text:match("(.*)\n\n(.*)")
    local yaml = yaml.load(yaml_text)
    local processed_body = body:gsub("\n", "\\n")
    local html_body = markdown_to_html(processed_body, {safe = true})

    local params = {
        uuid = id,
        type = yaml.type,
        title = yaml.title,
        body = html_body,
        created = yaml.created or "",
        updated = yaml.updated or "",
    }

    return params
end

return {
    split_document = split_document,
}
