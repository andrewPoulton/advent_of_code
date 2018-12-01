module Utils

export readInput

function readInput(day::Int64)
    fname = "inputs/day$day.txt"
    local outArray
    open(fname, "r") do f
        outArray = readlines(f)
    end
    return outArray
end

end