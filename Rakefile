# frozen_string_literal: true

task :default do
  puts <<EOS
Usage: rake TASK

Available tasks:
  test      Run all test specs.
EOS
end

task :test do
  sh 'cargo test'
  sh 'rspec --format documentation ./test/spec/'
end
