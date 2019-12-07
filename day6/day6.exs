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
end

File.read!("./signal.txt")
|> Day6.build_tree
|> Day6.fetch
|> IO.inspect