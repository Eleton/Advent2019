defmodule Day15 do
  def init(input) do
    input
    |> String.split(",")
    |> Enum.map(&(String.to_integer(&1)))
    # |> evaluate
    |> super_evaluate
  end
  def evaluate(list) do
    evaluate(Enum.reverse(list), length(list), MapSet.new(list))
  end
  def evaluate([h|_], 20, _)do
    h
  end
  def evaluate([h|t] = list, count, set) do
    if MapSet.member?(set, h) do
      new_list = [search(h, t) | list]
      evaluate(new_list, count + 1, set)
    else
      new_set = MapSet.put(set, h)
      evaluate([0 | list], count + 1, new_set)
    end
  end
  
  def search(s, list) do
    search(s, list, 1)
  end
  def search(s, [s|_t], acc) do
    acc
  end
  def search(_, [], _) do
    0
  end
  def search(s, [_|t], acc) do
    search(s, t, acc + 1)
  end

  def super_evaluate(list) do
    len = length(list)
    {map, _} = list
    |> Enum.reduce({Map.new(), 1}, fn n, {m, i} -> {Map.put(m, n, [i]), i+1} end)
    
    {_, last} = (len + 1)..(30000000 - 1)
    |> Enum.reduce({map, 0}, fn i, {m, next} ->
      if Map.has_key?(m, next) do
        list = Map.get(m, next)
        [h | _] = list
        n = i - h
        {Map.put(m, next, [i | list]), n}
      else
        {Map.put(m, next, [i] ), 0}
      end
    end)
    last
  end
end

input = "17,1,3,16,19,0"
Day15.init(input)
|> IO.inspect