$: << "."
$: << ".."
require "rubyfmt.so"
require "build/rubyfmt"
require "json"


#start_time = Time.now.to_f
#100.times {
#  file_data = File.read("../fixtures/rspec_core_notifications_actual.rb")
#  inspected_parsed = JSON.dump(Parser.new(file_data).parse)
#  Rubyfmt::format_to_stdout(file_data, inspected_parsed)
#}
#end_time = Time.now.to_f
#p((end_time - start_time) / 100)
#
#
file_data = "def a\n  b\nend\n"
inspected_parsed = JSON.dump(Parser.new(file_data).parse)
Rubyfmt::format_to_stdout(file_data, inspected_parsed)