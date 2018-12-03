using Utils
using Printf

incr(i::Int64) = (i % 4) + 1
decr(i::Int64) = i == 1 ? 4 : ((i - 1) % 4)

function nextDirection(old::Int64, turn::Char)
    if turn == 'R'
        return incr(old)
    else
        return decr(old)
    end
end

function day1part1(directions)
    current_direction = 1
    journey = [0,0,0,0] #Dist travelled [North, East, South, West]
    for direction in directions
        current_direction = nextDirection(current_direction, direction[1])
        journey[current_direction] += parse(Int, direction[2:end])
    end
    vert_dist = journey[1] - journey[3]
    hoz_dist = journey[2] - journey[4]
    return abs(vert_dist) + abs(hoz_dist)
end

directions = split(readInput(1)[1], ", ")

@time day1part1(directions)
@printf "The answer to part one is %d blocks\n" day1part1(directions)

loc(journey) = [journey[1] - journey[3], journey[2] - journey[4]]

function day1part2(directions)
    current_location = [0,0]
    been = push!(Set(), location)
    current_direction = 1
    journey = [0,0,0,0]
    for dir in directions
        current_direction = nextDirection(current_direction, dir[1])
        journey[current_direction] += parse(Int, dir[2:end])
        new_location = loc(journey)
        println(location)
        if current_direction == 1 #North
            deltay = new_location[2] > current_location ? new_location[2]:current_location : current_location:new_location[2]
            location
            if in(location, been)
                return location
            else
                push!(been, location)
            end
        current_location = new_location
    end
    return "dunno"
end
directions = split("R8, R4, R4, R8", ", ")
day1part2(directions)

function hasRevisited(start, finish, been_dict)


end

function day1part2_(directions)
    been = Dict{Int64, Array{Int64}}()
    current_direction = 1
    location = [0,0]
    journey = [0,0,0,0]
    for dir in directions
        current_direction = nextDirection(current_direction, dir[1])
        journey[current_direction] += parse(Int, dir[2:end])
        new_location = loc(journey)
end
