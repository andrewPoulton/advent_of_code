using Utils

testdata = split("""1, 1
1, 6
8, 3
3, 4
5, 5
8, 9""", "\n")

println(testdata)
println(sort(testdata))

ipt = readInput(6)

manhattenDist(x, y) = abs(x[1] - y[1]) + abs(x[2]- y[2])
strtopoint(x) = map((y)-> parse(Int,y), split(x,", "))
@time map((x)->strtopoint(x), testdata)
f = map((x)->strtopoint(x), ipt)
f_sort = map((x)->strtopoint(x), sort(ipt))
# for i in 1:(length(f)-1)
#     for j in (i+1):length(f)
#         println(manhattenDist(f[i], f[j]), ".....", f[i], f[j])
#     end
# end

function nearestpt(point, points)
    d = 10000000
    local out
    for (i, p) in enumerate(points)
        dist = manhattenDist(point, p)
        # println(dist, p)
        if dist < d
            out = i
            d = dist
        end
    end
    return points[out]
end

# @time nearestpt([3,7], f)
# println(nearestpt([3,7], f))



# function findCorners(points)
#     corners = [points[1:3]]
#     for p in points[3:end]



# end


function printGrid(points)
    min_x = minimum([p[1] for p in points])
    min_y = minimum([p[2] for p in points])
    max_x = maximum([p[1] for p in points])
    max_y = maximum([p[2] for p in points])
    width = max_x - min_x + 1
    height = max_y - min_y + 1
    println(width, height)
    for j in 1:height
        for i in 1:width
            if in([i+min_x-1,j+min_y-1], points)
                print("*")
            else
                print(".")
            end
        end
        print("\n")
    end
end

printGrid(f)