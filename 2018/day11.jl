using Utils

hundreds(x) = Int(((x - (x %100))//100)%10)

function fill_value(i,j,n)
    val = i + 10
    val *= j
    val += n
    val *= i+10
    return hundreds(val) - 5
end

create_matrix(x,y,n) = [fill_value(i,j,n) for i in 1:x, j in 1:y]

subsum(i,j,m,s) = sum(m[i:i+s-1,j:j+s-1])

function get_max_coord(m)
    current_max = -10000000
    out = [0,0]
    for i in 1:(size(m)[1] - 2)
        for j in 1:(size(m)[2] - 2)
            s = subsum(i,j,m)
            if s > current_max
                out = [i,j]
                current_max =  s
            end
        end
    end
    return out, current_max
end

function get_max_coord(m, l)
    current_max = -10000000
    out = [0,0]
    for i in 1:(size(m)[1] - l + 1)
        for j in 1:(size(m)[2] - l + 1)
            s = subsum(i,j,m,l)
            if s > current_max
                out = [i,j]
                current_max =  s
            end
        end
    end
    return out, current_max
end