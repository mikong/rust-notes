require 'ffi'

module Main
  extend FFI::Library
  ffi_lib 'target/debug/librubyffi.' + FFI::Platform::LIBSUFFIX
  attach_function :add, [:int, :int], :int
end

a, b = 2, 3
sum = Main.add(a, b)
puts "#{a} + #{b} = #{sum}"
