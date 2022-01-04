# frozen_string_literal: true

require 'fileutils'
require 'open3'
require 'stringio'

module AppHelper
  @@testdir = File.dirname(File.absolute_path(__FILE__))

  # The 'test' directory.
  def self.testdir
    @@testdir
  end

  @@topdir = File.dirname(@@testdir)

  # The project directory.
  def self.topdir
    @@topdir
  end

  @@program = "#{@@topdir}/target/debug/rmarshal"

  # The program to test.
  def self.program
    @@program
  end

  @@datadir = "#{@@testdir}/data"

  # The 'test/data' directory.
  def self.datadir
    @@datadir
  end

  @@tmpdir = "#{@@testdir}/tmp"

  # The 'test/tmp' directory.
  def self.tmpdir
    @@tmpdir
  end

  class Context
    attr_reader :datadir

    def initialize(scope)
      # The scoped 'test/data' directory.
      @datadir = "#{AppHelper.datadir}/#{scope}"
    end
  end

  def self.new_context(scope)
    Context.new(scope)
  end

  # Creates a directory if it does not exist yet.
  def self.make_dir(path)
    return if File.directory?(path)

    FileUtils.mkdir(path)
  end

  # Removes all files of a given directory.
  def self.clear_dir(path)
    Dir.entries(path).each do |entry_name|
      if entry_name == '.' || entry_name == '..'
        next
      end

      FileUtils.rm("#{path}/#{entry_name}")
    end
  end

  # Executes a program.
  # @param args [Array<String>]
  # @param opts [Hash]
  # @option opts [IO] input
  # @option opts [IO] output
  def self.exec_prog(args, opts = {})
    input = opts.fetch(:stdin, nil)
    use_stdin = !input.nil?
    output = opts.fetch(:stdout, nil)
    use_stdout = !output.nil?

    capture_opts = {}
    capture_opts[:stdin_data] = input.string if use_stdin
    stdout_data, exit_status = Open3.capture2(AppHelper.program, *args, capture_opts)
    if use_stdout
      output.write stdout_data
    end

    raise RuntimeError, "exit_status: actual(#{exit_status}) != expected(0)" if exit_status != 0

    exit_status
  end
end
