class Fibonacci
  include Enumerable

  def initialize
    @a = 0
    @b = 1
  end

  def each
    loop do
      yield @a
      tmp = @a
      @a = @b
      @b = @a + tmp
    end
  end
end


puts Fibonacci.new
  .take(10)
  .reject { |number| number.even? }
  .zip(Fibonacci.new.take(10))
  .map {|(a, b)| a * b }
  .reduce { |total, number| total + number }