local doc = select.document(html)

print("# Menu")
for key,e in ipairs(doc:find(select.descendant(select.attr("id", "hmenus"), select.name("a")))) do
    print(e:text(), e:attr("href"))
end
print("")

print("# Top 5 questions")
local items = doc:find(select.class("question-summary"))
for i = 1, 5 do
    if items[i] ~= nil then
        local question = items[i]:find(select.class("question-hyperlink"))[1]
        local votes = items[i]:find(select.class("vote-count-post"))[1]:text()
        local answers = items[i]:find(select.descendant(select.class("status"), select.name("strong")))[1]:text()
        local asked_on = items[i]:find(select.class("relativetime"))[1]:text()
        local asker = items[i]:find(select.descendant(select.class("user-details"), select.name("a")))[1]:text()
        local tags = items[i]:find(select.class("post-tag"))
        for key,value in ipairs(tags) do
            tags[key] = value:text()
        end

        print("Question: ", question:text())
        print("Votes: ", votes)
        print("Answers: ", answers)
        print("Asked on: ", asked_on)
        print("Asked by: ", asker)
        print("Tags: ", table.concat(tags, ", "))
        print("Permalink: http://stackoverflow.com" .. question:attr("href"))
        print("")
    end
end

print("# Top 10 Related Tags")
local x = doc:find(select.attr("id", "h-related-tags"))[1]:parent():find(select.name("div"))
for i = 1, 10 do
    if x[i] ~= nil then
        local tag = x[i]:find(select.name("a"))[1]:text()
        local count = x[i]:find(select.class("item-multiplier-count"))[1]:text()
        print(tag, count)
    end
end
