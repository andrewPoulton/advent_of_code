using Utils

mutable struct Vertex
    id::String
    i::Int
    j::Int
    traversable::Bool
    free::Bool
end

Vertex(i::Int, j::Int, traversable::Bool) = Vertex("V_$i" * "_$j", i, j, traversable, true)

struct Edge
    s::Vertex
    e::Vertex
end

mutable struct Graph
    vertices::Vector{Vertex}
    edges::Vector{Edge}
end

mutable struct Being
    id::String
    location::Vertex
    hp::Int
end


isNeighbour(v1::Vertex, v2::Vertex) = (v2.j == v1.j && in(v2.i, [v1.i + 1, v1.i - 1])) || (v2.i == v1.i && in(v2.j, [v1.j + 1, v1.j - 1]))
neighbours(v::Vertex, g::Graph) =  map(e->e.e, filter(e->e.s == v, g.edges))
freeNeighbours(v::Vertex, g::Graph) =  map(e->e.e, filter(e->e.s == v && e.e.free, g.edges))


function buildGraphGetBeings(input)
    graph = Graph([],[])
    beings = []
    for (i, row) in enumerate(input)
        for (j, v) in enumerate(row)
            if v!='#'
                new_vertex = Vertex(i, j, true)
                if in(v, "GE")
                    k = length(beings)
                    id = v * "_$k"
                    new_vertex.free = false
                    new_being = Being(id, new_vertex, 200)
                    push!(beings, new_being)
                end
                push!(graph.vertices, new_vertex)
            end
        end
    end
    for v in graph.vertices
        neighbours = filter(x->isNeighbour(v,x), graph.vertices)
        for neighbour in neighbours
            new_edge = Edge(v, neighbour)
            push!(graph.edges, new_edge)
        end
    end
    return graph, beings
end



min_val(d) = collect(keys(d))[argmin(collect(values(d)))]

function Dijkstra(graph::Graph, source::Vertex, target::Vertex)
    Q = Dict() #unvisited nodes
    dist = Dict()
    prev = Dict()
    for (i, v) in enumerate(graph.vertices)
        dist[v.id] =  v == source ? 0 : 10000
        prev[v.id] = nothing
        Q[v.id] = v
    end
    iters = 0
    while length(Q) > 0
        filtered_dists = filter(x->x[1] in keys(Q), dist)
        min_dist_vertex = min_val(filtered_dists)
        d = dist[min_dist_vertex]
        # if d == 10000
        #     return []
        # end
        u = Q[min_dist_vertex]
        for neighbour in freeNeighbours(u, graph)
            alt = dist[u.id] + 1
            if alt < dist[neighbour.id]
                dist[neighbour.id] = alt
                prev[neighbour.id] = u
            end
        end
        # pop!(dist, min_dist_vertex)
        pop!(Q, min_dist_vertex)
        # if u == target
        #     println("Reached target vertex " * target.id)
        #     break
        # end
        iters +=1
    end
    # println("Visited $iters nodes")
    u = target
    if prev[u.id] == nothing
        return [], [], []
    end
    sequence = []
    while u != nothing
        pushfirst!(sequence, u)
        u = prev[u.id]
    end
    return sequence, filter(x->x[2] != nothing, collect(prev)), dist
end

function find_targets(target_type, beings, graph)
    targets = filter(x->x.id[1] == target_type, beings)
    potential_locations = Set()
    for target in targets
        union!(potential_locations, freeNeighbours(target.location, graph))
    end
    return collect(potential_locations)
end

function nearest_target(being, targets, graph)
    path = []
    dist = 10000
    for target in targets
        target_path = Dijkstra(graph, being.location, target)[1]
        target_dist = length(target_path)
        if 0 < target_dist < dist
            dist = target_dist
            path = target_path
        end
    end
    return 0 < dist < 10000 ? path : []
end

function target_getter(being_channel, results_channel, targets, graph)
    for being in being_channel
        path = nearest_target(being, targets, graph)
        put!(results_channel, path)
    end
end

function can_attack(being, beings, graph)
    being_type = being.id[1]
    targets = map(b->b.location, filter(x->x.id[1] != being_type, beings))
    targets = filter(x->x in targets, neighbours(being.location, graph))
    if length(targets) > 0
        sort!(targets, by=x->(x.i,x.j))
        return targets[1]
    end
    return nothing 
end

function attack(attacker, target)
    target.hp -= 3
end


function take_turn(being, beings, targets, graph)
    target = can_attack(being, beings, graph)
    if target != nothing
        attack(being, target)
        return "Attacked target  " * target.id
    end
    target_path = nearest_target(being, targets, graph)

    if length(target_path) == 0
        return "Can\'t move"
    end

    being.location.free = true
    being.location = target_path[2]
    being.location.free = false
    target = can_attack(being, beings, graph)

    if target != nothing
        attack(being, target)
        return return "Moved to " * being.location.id * " and attacked target " * target.id
    end
    return "Moved to " * being.location.id
end

function highlight_locations(locations)
    map_ = readInput(15,2018)
    for (i, location) in enumerate(locations)
        row = collect(map_[location.i])
        k = i % 10
        row[location.j] = "$k"[1]
        map_[location.i] = join(row)
    end
    return map_
end

function draw_map()
    map_ = readInput(15, 2018)
    map_ = map(line -> replace(line, r"G|E"=>"."), map_)
    return map_
end

function place_beings(beings)
    map_ = draw_map()
    for being in beings
        row = collect(map_[being.location.i])
        k = being.id[1]
        row[being.location.j] = k 
        map_[being.location.i] = join(row)
    end
    return map_
end
    


function main()
    input = readInput(15, 2018)
    graph, beings = buildGraphGetBeings(input)
    source = graph.vertices[16]
    target = graph.vertices[45]
    trip = Dijkstra(graph, source, target)
    println(trip)
    return graph, beings, source, target
end

graph, beings, source, target = main()