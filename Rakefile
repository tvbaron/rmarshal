# frozen_string_literal: true

task :default do
  puts <<EOS
Usage: rake TASK

Available tasks:
  test      Run all test specs.
EOS
end

desc 'Compile the debug target.'
task :compile_debug do
  sh 'cargo build'
end

desc 'Run all tests.'
task :test => :compile_debug do
  sh 'cargo test'
  sh 'rspec --format documentation ./test/spec/'
end
