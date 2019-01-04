using Utils: readInput


numbers = r"-?\d+"

function parseLine(line::String)
    matches = eachmatch(numbers, line)
    return map(x->parse(Int, x.match), matches)
end

function getLocsandVs(parsedLines)
    m = reshape(vcat(parsedLines...), (4, 341))
    return m[1:2,:], m[3:4,:]
end

function getGridArea(locations)
    x = locations[1,:]
    y = locations[2,:]
    return (maximum(x) - minimum(x))* (maximum(y) - minimum(y))
end

function getGrid(locations)
    x = locations[1,:]
    y = locations[2,:]
    return maximum(x),  minimum(x), maximum(y), minimum(y)
end

function draw(locs)
    xmax, xmin, ymax, ymin = getGrid(locs)
    xs = locs[1,:]
    ys = locs[2,:]
    ln = ""
    grid = zeros(Int, (ymax-ymin + 1, xmax-xmin + 1))
    for i in 1:length(xs)
        x = xs[i] - xmin + 1
        y = ys[i] - ymin + 1
        grid[y,x] = 1
    end
    print('\n')
    for i in 1:(ymax-ymin + 1)
        for j in 1:(xmax-xmin)
            ln *= grid[i,j] == 1 ? "#" : " "
        end
            println(ln)
            ln = ""
    end
    print('\n')
end

function tick(s,l,v)
    return l .+ (s .* v)
end


function main()
    input = readInput(10, 2018)
    parsed = map(parseLine, input)
    locations, vel = getLocsandVs(parsed)
    for i in 1:15000
        l = tick(i, locations, vel)
        if getGridArea(l) <  1000
            locations = l
            break
        end
    end
    draw(locations)
end


@time main()