function minifyjavascript(js::String)
    :String
    str = strip(js, '\t')
    str = strip(str, '\n')
    str = strip(str, '\r')

    str = replace(str, Pair(" = ", "="))

    return remove_repeating_semis(str)
end

function remove_repeating_semis(js::String)
    :String
    str = js
    # check for repeating ';'
    while contains(str, ";;")
        str = replace(str, Pair(";;", ";"))
    end
    return str
end