using Utils


mutable struct treeNode
    name::Char
    children::Set
    parents::Set
    visited::Bool
end

struct Tree
    Nodes::Vector{treeNode}
    nodeNames::Dict
end

treeNode(x::Char) = treeNode(x, Set{Char}(), Set{Char}(), false)
Tree() = Tree([], Dict{Char, Int64}())

function findNode(tree::Tree, node::Char)
    return haskey(tree.nodeNames, node)
end

function node(tree, key)
    if haskey(tree.nodeNames, key)
        idx = tree.nodeNames[key]
        return tree.Nodes[idx]
    else
        return nothing
    end
end


function addChild!(tree::Tree, parent::Char, child::Char)
    pnode = node(tree, parent)
    cnode = node(tree, child)
    if pnode != nothing
        push!(pnode.children, child)
    else
        newnode = treeNode(parent, Set{Char}(child), Set{Char}(), false)
        push!(tree.Nodes, newnode)
        tree.nodeNames[parent] = length(tree.Nodes)
    end
    if cnode != nothing
        push!(cnode.parents, parent)
    else
        newnode = treeNode(child, Set{Char}(),  Set{Char}(parent), false)
        push!(tree.Nodes, newnode)
        tree.nodeNames[child] = length(tree.Nodes)
    end
end

function parseLine(line)
    splitLine = split(line, ' ')
    return [splitLine[2][1], splitLine[8][1]]
end



function buildTree(input)
    input = map(parseLine, input)
    local tree = Tree()
    for node in input
        addChild!(tree, node[1], node[2])
    end
    sort!(tree.Nodes, by=x->x.name)
    return tree
end

function getVisited(tree::Tree)
    return map(x->x.name, filter(x->x.visited, tree.Nodes))
end

function getUnvisited(tree::Tree)
    return map(x->x.name,filter(x->!x.visited, tree.Nodes))
end

input = readInput(7)

function findFree(tree, ignore=Set(), workers = 1)
    candidates = filter(x->intersect(x.parents, ignore) == x.parents, tree.Nodes)
    filter!(x->!in(x.name, ignore), candidates)
    m = minimum([length(candidates), workers])
    return candidates[1:m]
end


function walk(tree, workers)
    u = getUnvisited(tree)
    v = Set()
    out = []
    its = 0
    while(length(u) > 0)
        freeNodes = findFree(tree, v, workers)
        for x in freeNodes
            x.visited = true
            push!(out, x.name)
            # println(x.name)
            push!(v, x.name)
        end
        u = getUnvisited(tree)
        # println(length(u))
        its +=1
    end  
    for w in out
        print(w)
    end
    print('\n')
    println(its)
    map(x->x.visited = false, tree.Nodes)
    return out
end
