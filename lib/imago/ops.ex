defmodule Imago.Ops do
  defstruct list: []

  def new() do
    %Imago.Ops{}
  end

  @allowed %{
    open: 1,
    resize: 2,
    dither: 2,
    save: 1,
    copy: 1
  }

  def append(ops, {:open, _args} = op),
      do: ops
          |> ops_is_first?
          |> ap(op)
  def append(ops, {:resize, _args} = op),
      do: ops
          |> ops_has_image?
          |> ap(op)
  def append(ops, {:dither, _args} = op),
      do: ops
          |> ops_has_image?
          |> ap(op)
  def append(ops, {:save, _args} = op),
      do: ops
          |> ops_has_image?
          |> ops_has_manipulation?
          |> ap(op)
  def append(ops, {:copy, _args} = op),
      do: ops
          |> ops_has_image?
          |> ap(op)


  @doc """
  Checks that opening an image is the first operation

      iex> Imago.Ops.ops_is_first?(%{list: [{:open, ["arg"]}]})
      %{list: [{:open, ["arg"]}]}

      iex> Imago.Ops.ops_is_first?(%{list: [{:resize, ["arg"]}]})
      ** (Imago.Error) Opening an image must be the first operation of the pipeline.
  """
  def ops_is_first?(%{list: [{:open, _}]} = ops), do: ops
  def ops_is_first?(_), do: raise Imago.Error, message: "Opening an image must be the first operation of the pipeline."


  @doc """
  Checks that an image has been opened

      iex> Imago.Ops.ops_has_image?(%{list: [{:open, ["arg"]}, {:resize, ["arg"]}]})
      %{list: [{:open, ["arg"]}, {:resize, ["arg"]}]}

      iex> Imago.Ops.ops_has_image?(%{list: [{:resize, ["arg"]}]})
      ** (Imago.Error) No image has been opened in the ops pipeline.
  """
  def ops_has_image?(ops) do
    case Enum.find(ops.list, &(elem(&1, 0) == :open)) do
      nil -> raise Imago.Error, message: "No image has been opened in the ops pipeline."
      _ -> ops
    end
  end

  @doc """
  Checks that an image has been manipulated before saving it

      iex> Imago.Ops.ops_has_manipulation?(%{list: [{:open, ["arg"]}, {:resize, ["arg"]}, {:save, ["arg"]}]})
      %{list: [{:open, ["arg"]}, {:resize, ["arg"]}, {:save, ["arg"]}]}

      iex> Imago.Ops.ops_has_manipulation?(%{list: [{:resize, ["arg"]}, {:save, ["arg"]}]})
      ** (Imago.Error) No manipulation has been done on the image. Use :copy instead of :save.
  """
  def ops_has_manipulation?(ops) do
    if ops.list
       |> length > 2 do
      ops
    else
      raise Imago.Error, message: "No manipulation has been done on the image. Use :copy instead of :save."
    end
  end

  @doc """
  Checks for valid operations & argument number in the operations pipeline.

      iex> Imago.Ops.is_pipeline_valid?(%{list: [{:open, ["arg"]}]})
      %{list: [{:open, ["arg"]}]}

      iex> Imago.Ops.is_pipeline_valid?(%{list: [{:open, ["arg", "arg2"]}]})
      ** (Imago.Error) Operations pipeline is in an invalid state. Run aborted.
  """
  def is_pipeline_valid?(ops) do
    case Enum.reduce(
           ops.list,
           true,
           fn {op, args}, flag -> flag and (Map.has_key?(@allowed, op) and Map.get(@allowed, op) == length(args)) end
         ) do
      true -> ops
      _ -> raise Imago.Error, message: "Operations pipeline is in an invalid state. Run aborted."
    end
  end

  defp ap(ops, op), do: %{ops | list: ops.list ++ [op]}
end