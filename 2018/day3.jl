using Utils
using Printf
claims = readInput(3)
# claims =  ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4","#3 @ 5,5: 2x2"]

function day3bothparts(claims)
    A = zeros(Int64, 1000, 1000)
    overlapping_squares = 0
    overlapped = zeros(Int64, length(claims))
    for (i, claim) in enumerate(claims)
        rects = getRectangle(claim)
        for rect in rects
            square = A[rect[1], rect[2]]
            if square == -1
                #Already multiply claimed square
                overlapped[i] = 1
                continue
            elseif square == 0
                #First claim of square
                A[rect[1], rect[2]] = i
            else
                #Second claim of square
                A[rect[1], rect[2]] = -1
                overlapping_squares += 1
                overlapped[square] = 1
                overlapped[i] = 1
            end
        end
    end
    return overlapping_squares, findfirst(x-> x==0, overlapped)
end

answer1, answer2 = day3bothparts(claims)
@printf "The answer to part one is %d\n" answer1
@printf "The answer to part two is %d\n" answer2
@time day3bothparts(claims)