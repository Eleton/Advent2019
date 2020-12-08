defmodule Day7 do
  def read do
    File.read!("signal.txt")
    |> String.split("\n")
  end

  def build_tree(rules, root) do
    {:ok, rex1} = Regex.compile(".+" <> root)
    {:ok, rex2} = Regex.compile("^[[:word:]]+ [[:word:]]+")
    list = rules
    |> Enum.filter(&(Regex.match?(rex1, &1)))
    |> Enum.map(&(Regex.run(rex2, &1)))
    |> List.flatten

    children = list
    |> Enum.map(&(build_tree(rules, &1)))
    {root, children}
  end

  def traverse(tree) do
    traverse(tree, MapSet.new())
  end
  def traverse({bag, []}, set) do
    MapSet.put(set, bag)
  end
  def traverse({bag, children}, set) do
    new_set = MapSet.put(set, bag)
    Enum.reduce(children, new_set, &(traverse(&1, &2)))
  end

  def build_tree_again(rules, root, n) do
    {:ok, rex1} = Regex.compile("^" <> root)
    {:ok, rex2} = Regex.compile("([[:digit:]]+) ([[:word:]]+ [[:word:]]+)")
    list = rules
    |> Enum.filter(&(Regex.match?(rex1, &1)))
    |> Enum.map(&(Regex.scan(rex2, &1)))
    |> hd
    |> Enum.map(fn [_, n, bag] -> {String.to_integer(n), bag} end)

    children = list
    |> Enum.map(fn {n, new_bag} -> build_tree_again(rules, new_bag, n) end)
    {root, n, children}
  end

  def traverse_again({_bag, n, []}) do
    n
  end
  def traverse_again({_bag, n, children}) do
    n + n*Enum.reduce(children, 0, fn child, acc -> acc + traverse_again(child) end)
  end
end

rules = Day7.read()
rules
|> Day7.build_tree("shiny gold")
|> Day7.traverse
|> MapSet.difference(MapSet.new(["shiny gold"]))
|> MapSet.size
|> IO.inspect

size = rules
|> Day7.build_tree_again("shiny gold", 1)
|> Day7.traverse_again

IO.inspect(size - 1)