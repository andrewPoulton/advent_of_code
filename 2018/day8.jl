using Utils

i = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"

struct header
    children::Int
    meta::Int
end

struct node
    header::header
    children::Vector{node}
    meta::Array
end

function parseTree(input)
    startPointer = 2
    endPointer = length(input)
    while endPointer > startPointer
    end
end