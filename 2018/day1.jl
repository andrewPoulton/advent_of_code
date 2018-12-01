using Printf
using Utils: readInput
fname="inputs/day1.txt"

#Part 1
freqs = []
start = 0
frequencies = map((x) -> parse(Int,x), readInput(1))
open(fname,"r") do f
    global start
    for line in eachline(f)
        val = parse(Int, line)
        push!(freqs, val)
        start+=val
    end
end
@printf "The answer to part one is %d\n" sum(freqs)
@printf "The answer to part one is also %d\n" start
@printf "The answer to part one is also possibly %d\n" sum(frequencies)


#Part 2
seen = Dict{Int64, Int64}()
keep_going = true
current_freq = 0
while keep_going
    global current_freq
    global keep_going
    for (i, freq) in enumerate(freqs)
        current_freq += freq
        if haskey(seen, current_freq)
            println("Answer to part two is $current_freq, seen first at index $i")
            keep_going = false
            break
        else
            seen[current_freq] = i
        end
    end
end