using Utils: readInput, counter, oneLetterDifferent
using Printf

function day2part1()
    ids = readInput(2)
    twos = 0
    threes = 0
    for id in ids
        counts = Set(values(counter(id)))
        twos += in(2, counts) ? 1 : 0
        threes += in(3, counts) ? 1 : 0
    end
    return twos * threes
end

@time day2part1()

@printf "Checksum is %d\n" day2part1()

function day2part2()
    ids = readInput(2)
    len = length(ids)
    for i in 1:len
        for j in i+1:len
            old = oneLetterDifferent(ids[i], ids[j])
            if old[1] == 1
                return string(ids[i][1:old[2]-1], ids[i][old[2]+1:end])
            end
        end
    end
    return "didn't find 'em"
end

@time day2part2()
println(day2part2())
