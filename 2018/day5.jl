using Utils


remv(s::String, i::Int) = s[1:i-2] * s[i+1:end]
function reducePolymer(polymer::String)
    prev_char = ""
    local _polymer
    while true
        idx = 0
        for (i, char) in enumerate(polymer)
            # println(prev_char)
            if uppercase(prev_char) == uppercase(char) && char != prev_char
                idx = i
                _polymer = remv(polymer, idx)
                # println(polymer)
                break
            end
            prev_char = char
        end
        if _polymer == polymer
            return _polymer
        else
            polymer = _polymer
        end
    end
end

r = "dabAcCaCBAcCcaDA"
println(length(reducePolymer(r)))
println(reducePolymer(readInput(5)[1]))