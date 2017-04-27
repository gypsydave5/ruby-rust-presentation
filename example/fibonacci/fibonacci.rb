require 'fiddle'
require 'benchmark'

def ruby_fibonacci(number)
    a = 0
    b = 1

    number.times do
        tmp = a
        a = b
        b = a + tmp
    end

    b
end


rust_fibonacci_library = Fiddle.dlopen('./fibonacci_rust/target/release/libfibonacci.dylib')
rust_fibonacci = Fiddle::Function.new(
  rust_fibonacci_library['fibonacci'],
  [Fiddle::TYPE_INT],
  Fiddle::TYPE_INT
)

c_fibonacci_library = Fiddle.dlopen('./fibonacci_c/fibonacci.dylib')
c_fibonacci = Fiddle::Function.new(
  c_fibonacci_library['fibonacci'],
  [Fiddle::TYPE_INT],
  Fiddle::TYPE_INT
)

10.times().each{|n| puts "Fibonacci #{n} is #{ruby_fibonacci(n)}"}
10.times().each{|n| puts "Fibonacci #{n} is #{c_fibonacci.call(n)}"}
10.times().each{|n| puts "Fibonacci #{n} is #{rust_fibonacci.call(n)}"}

n=2000
Benchmark.bmbm(10) do |x|
  x.report("ruby") { n.times{ ruby_fibonacci(n) }}
  x.report("c") { n.times{ c_fibonacci.call(n) }}
  x.report("rust") { n.times{ rust_fibonacci.call(n) }}
end