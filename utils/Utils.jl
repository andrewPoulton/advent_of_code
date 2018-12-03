module Utils

export readInput, counter, oneLetterDifferent, getRectangle

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

function getRectangle(claim::String)
    #Used in 2018 day3 part1
    left, top, width, height = map((x)->parse(Int,x), match(r"(\d+),(\d+): (\d+)x(\d+)", claim).captures)
    #Bloody 1-indexing
    exes = (left+1):(left+width)
    whys = (top+1):(top+height)
    out = []
    for x in exes
        for y in whys
            push!(out, (x,y))
        end
    end
    return out
end

end