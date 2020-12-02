defmodule Day6 do
  def build_tree(str) do
    str
    |> String.split("\n")
    |> Enum.map(&(String.split(&1, ")")))
    |> Day6.sort("COM")
    |> Enum.reduce({"COM", 0, []}, fn [parent, child], acc ->
      insert(acc, parent, child)
    end)
  end

  def sort([], _) do
    []
  end
  def sort(list, parent) do
    {sorted, rest} = Enum.split_with(list, fn [p, _] ->
      p === parent
    end)
    [[_, code] = h | t] = sorted ++ rest
    [h | sort(t, code)]
  end

  def insert({parent, distance, children}, parent, child) do
    {parent, distance, [{child, distance + 1, []} | children]}
  end
  def insert({parent, distance, []}, _code, _child) do
    {parent, distance, []}
  end
  def insert({parent, distance, children}, code, new_child) do
    {parent, distance, Enum.map(children, fn child ->
      insert(child, code, new_child)
    end)}
  end

  def fetch({_, distance, []}) do
    distance
  end
  def fetch({_, distance, children}) do
    distance + Enum.reduce(children, 0, fn child, acc ->
      acc + fetch(child)
    end)
  end

  def find_object({code, _, _}, code) do
    []
  end
  def find_object({_, _, []}, code) do
    nil
  end
  def find_object({parent, _, children}, code) do
    case find(children, code) do
      nil -> nil
      result -> [parent | result]
    end
    # path = Enum.find(children, fn c ->
    #   find_object(c, code)
    # end)
    # case path do
    #   nil -> nil
    #   c -> [parent | find_object(path, code)]
    # end
  end

  def find([], _) do
    nil
  end
  def find([child | children], code) do
    case find_object(child, code) do
      nil -> find(children, code)
      result -> result
    end
  end

  def evaluate_path([obj | san_tail], [obj | you_tail]) do
    IO.inspect(obj)
    evaluate_path(san_tail, you_tail)
  end
  def evaluate_path(san, you) do
    length(san) + length(you)
  end
end


path =
File.read!("./signal.txt")
# "COM)B
# B)C
# C)D
# D)E
# E)F
# B)G
# G)H
# D)I
# E)J
# J)K
# K)L
# K)YOU
# I)SAN"
|> Day6.build_tree

path_to_san = path
|> Day6.find_object("SAN")
|> IO.inspect

path_to_you = path
|> Day6.find_object("YOU")
|> IO.inspect

Day6.evaluate_path(path_to_san, path_to_you)
|> IO.inspect
