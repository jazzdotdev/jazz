local function read_file(path)
    local file = io.open(path, "r")

    if not file then
        return nil
    end

    local file_content = file:read("*all")
    file:close()
    return file_content
end


local function get_all_files_in(directory)
    local filenames = {}
    local pfile = io.popen('ls -p "'..directory..'"')

    for filename in pfile:lines() do
        if filename:sub(1, 1) ~= "/" then
            table.insert(filenames, filename)
        end
    end

    pfile:close()
    return filenames
end

return {
    read_file = read_file,
    get_all_files_in = get_all_files_in,
}
