function tryread(io, n)
    try
        return read(io, n)
    catch
        return ""
    end
end