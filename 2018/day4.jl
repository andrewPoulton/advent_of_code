using Dates
using Utils

testInput="""
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"""
# parseDate(line) = DateTime(replace(match(r"\[(.*)\]",line).captures[1], " " => "T"))
# @time map(parseDate, [date for date in split(testInput, "\n") if length(date) > 0])
# @time map(parseDate, readInput(4))

test = [date for date in split(testInput, "\n") if length(date) > 0]
function sleepSchedule(logs)
    logs = sort(logs)
    guardsAsleep = Dict{String, Array}()
    minute_regex = r"\d+(?=\])"
    local current_guard
    local minutes #sleep minutes per guard
    local time_asleep #when guard fell asleep
    local time_awake #when guard awoke
    for line in logs
        guard, asleep, awake = match(r"(\#\d+)|(asleep)|(wakes)", line).captures
        
        if guard != nothing #new guard begins shift
            current_guard = guard
        elseif asleep != nothing #guard has fallen asleep
            minute = match(minute_regex, line)
            time_asleep = parse(Int, minute.match)
        elseif awake != nothing
            minute = match(minute_regex, line)
            time_awake = parse(Int, minute.match)

            if !in(current_guard, keys(guardsAsleep))
                guardsAsleep[current_guard] = zeros(60)
            end
            for i in (time_asleep+1):time_awake
                guardsAsleep[current_guard][i] += 1
            end
        else
            return "the fuck happened"
        end
    end
    return guardsAsleep
end


function day4part1()
    ipt = readInput(4)
    # ipt = test
    sleep_sched = sleepSchedule(ipt)
    local sleepiest_guard 
    sleep_length = 0.
    sleepiest_minute = 0
    for (guard, sleep) in sleep_sched
        s = sum(sleep)
        if s > sleep_length
            sleepiest_minute = indexMax(sleep) - 1
            sleepiest_guard = guard
            sleep_length = s
        end
    end
    println("sleepiest guard in $sleepiest_guard, his sleepiest minute is $sleepiest_minute")
    return parse(Int, sleepiest_guard[2:end]) * sleepiest_minute
end
@time day4part1()
println(day4part1())

function day4part2()
    ipt = readInput(4)
    ss = sleepSchedule(ipt)
    _guard = ""
    sleepiest_minute = 0
    ms = 0.
    for (guard, sleep) in ss
        s = maximum(sleep)
        if s > ms
            sleepiest_minute = indexMax(sleep) - 1
            _guard = guard
            ms = s
            println(ms)
        end
    end
    println("sleepiest guard in $_guard, his sleepiest minute is $sleepiest_minute")
    return parse(Int, _guard[2:end]) * sleepiest_minute
end

println(day4part2())