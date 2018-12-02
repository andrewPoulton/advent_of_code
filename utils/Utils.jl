module Utils

export readInput, counter, oneLetterDifferent

function readInput(day::Int64)
    fname = "inputs/day$day.txt"
    local outArray
    open(fname, "r") do f
        outArray = readlines(f)
    end
    return outArray
end

function counter(text::String)
    out = Dict{Any, Any}()
    for l in text
        try
            out[l] += 1
        catch
            out[l] = 1
        end
    end
    return out
    end

function oneLetterDifferent(text1::String, text2::String)
    #Used in 2018 day2 part 2
    diff = 0
    len = length(text1)
    local diff_loc
    for i in 1:len
        if text1[i] != text2[i]
            diff += 1
            diff_loc = i
        end
        if diff > 1
            return [false, diff_loc]
        end
    end
    return [true, diff_loc]
end

end