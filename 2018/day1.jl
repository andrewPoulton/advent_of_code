using Printf
using Utils: readInput


function day1part1()
    frequencies = map((x) -> parse(Int64,x), readInput(1))
    answer = sum(frequencies)
    return "The answer to part one is also possibly $answer"
end


@time day1part1()
println(day1part1())

function day1part2()
    frequencies = map((x) -> parse(Int64,x), readInput(1))
    local current_freq = 0
    local seen = Set{Int64}()
    local keep_going = true
    local iters = 0
    while keep_going
        for (i, freq) in enumerate(frequencies)
            current_freq += freq
            if in(current_freq, seen)
                return "Answer is $current_freq, found at index $i, after $iters iterations"
            end
            push!(seen, current_freq)
        end
        iters += 1
    end
    return length(seen)
end

@time day1part2()
println(day1part2())