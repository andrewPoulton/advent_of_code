using Utils


recipe_scores = [3,7]
a_idx, b_idx = [1,2]


function new_recipe(x::Int64)
    out = [Int(floor(x // 10)), x % 10]
    return out[1] == 0 ? out[2:2] : out
end

function step_(arr, idx_a, idx_b)
    nr = new_recipe(arr[idx_a] + arr[idx_b])
    append!(arr, nr)
    l = length(arr)
    a = 1 + arr[idx_a]
    b = 1 + arr[idx_b]
    idx_a = idx_a + (a % l)
    idx_b = idx_b + (b % l)
    return arr, idx_a > l ? idx_a % l : idx_a, idx_b > l ? idx_b %l : idx_b
end


function main(input)
    global recipe_scores, a_idx, b_idx
    while length(recipe_scores) <= input + 10
        recipe_scores, a_idx, b_idx = step_(recipe_scores, a_idx, b_idx)
        # println(recipe_scores, a_idx, b_idx)
    end
    println(recipe_scores[input+1:input+10])
end

main(190221)