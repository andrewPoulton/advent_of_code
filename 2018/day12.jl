using Utils

input = readInput(12, 2018)

test_input = split("""initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #""",'\n')

function preprare_input(input)
    initial_state = split(input[1])[3]
    rules = Dict()
    for line in input[3:end]
        rule = split(line, " => ")
        rules[rule[1]] = rule[2][1]
    end
    return initial_state, rules
end

function replace(string, char, loc)
    x = collect(string)
    x[loc] = char
    return join(x)
end

function check_plant(loc, state, rules)
    # if 
    state = ".." * state
    state *= ".."

    if loc < 3
        rule = state[1:loc+2]
    else
        rule = state[loc-2:loc+2]
    end
    if haskey(rules,rule)
        return loc, rules[rule], rule
    end
    return nothing
end

function main(input)
    istate, rules = preprare_input(input)
    l = length(istate)
    println("currently $l long")
    for iter in 1:20
        println(iter)
        changes = map(i->check_plant(i, istate, rules), 1:(length(istate)-2))
        filter!(x->x != nothing, changes)
        println(length(changes))
        for change in changes
            istate = replace(istate, change[2], change[1])
        end
    end
    return istate
end
