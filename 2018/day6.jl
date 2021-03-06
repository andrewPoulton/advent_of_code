using Utils

testdata = split("""1, 1
1, 6
8, 3
3, 4
5, 5
8, 9""", "\n")

println(testdata)
println(sort(testdata))

ipt = readInput(6, 2018)

struct Pt
    x::Int64
    y::Int64
end

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

ccw(p1,p2,p3) = (p2[1]-p1[1])*(p3[2]-p1[2]) - (p2[2]-p1[2])*(p3[1]-p1[1])
dot(x,y) = sum(x.*y)/sqrt(sum(x.^2) * sum(y.^2))
function graham(points)
    min_y = [-1000,-1000]
    for p in points
        if p[2] > min_y[2]
            min_y = p
        end
    end
    sort!(points, by = x->dot(x,min_y), rev=true)
    ch = []
    push!(ch, points[1])
    push!(ch, points[2])
    for i in 2:(length(points)-1)
        while length(ch) >=2 && ccw(ch[end-1], ch[end], points[i]) <= 0
            pop!(ch)
        end
        push!(ch,points[i])
    end
    return ch
end
println(graham(f))

function printGrid(points)
    min_x = minimum([p[1] for p in points])
    min_y = minimum([p[2] for p in points])
    max_x = maximum([p[1] for p in points])
    max_y = maximum([p[2] for p in points])
    width = max_x - min_x + 1
    height = max_y - min_y + 1
    println(width, height)
    ch = graham(points)
    for j in 1:height
        for i in 1:width
            if in([i+min_x-1,j+min_y-1], ch)
                print("A")
            elseif in([i+min_x-1,j+min_y-1], points)
                print("*")
            else
                print(".")
            end
        end
        print("\n")
    end
end

printGrid(f)