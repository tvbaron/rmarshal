# frozen_string_literal: true

class Report
  attr_reader :test_total_cnt
  attr_reader :test_passed_cnt
  attr_reader :test_failed_cnt

  def initialize(opts = {})
    @test_total_cnt = opts.fetch(:test_total_cnt, 0)
    @test_passed_cnt = opts.fetch(:test_passed_cnt, 0)
    @test_failed_cnt = opts.fetch(:test_failed_cnt, 0)
  end

  def add_test_passed
    @test_total_cnt += 1
    @test_passed_cnt += 1
  end

  def add_test_failed
    @test_total_cnt += 1
    @test_failed_cnt += 1
  end

  def merge(other)
    Report.new( \
      :test_total_cnt => @test_total_cnt + other.test_total_cnt, \
      :test_passed_cnt => @test_passed_cnt + other.test_passed_cnt, \
      :test_failed_cnt => @test_failed_cnt + other.test_failed_cnt)
  end
end

class SpecError < RuntimeError
  def initialize(message)
    super(message)
  end
end

class Test
  def initialize(engine)
    @engine = engine
    @values = {}
    @values[:basedir] = engine.spec_dir
  end

  def get(key)
    @values[key]
  end

  def set(key, value)
    @values[key] = value
  end

  def make_dir(path)
    return if File.directory?(path)

    FileUtils.mkdir(path)
  end

  def clear_dir(path)
    Dir.entries(path).each do |entry_name|
      if entry_name == '.' || entry_name == '..'
        next
      end

      FileUtils.rm("#{path}/#{entry_name}")
    end
  end

  def exec_prog(args, opts = {})
    input = opts.fetch(:stdin, nil)
    use_stdin = !input.nil?
    output = opts.fetch(:stdout, nil)
    use_stdout = !output.nil?

    Open3.popen3(@engine.program, *args) do |stdin, stdout, stderr, wait_thr|
      if use_stdin
        stdin.puts input.string
        stdin.close
      end
      if use_stdout
        while line = stdout.gets
          output.puts line
        end
        output.close
      end

      # while line = stderr.gets
      #   $stderr.puts line
      # end

      exit_status = wait_thr.value
      raise SpecError, "exit_status = #{exit_status}" if exit_status != 0
    end
  end

  def assert(&block)
    raise SpecError, 'assert' unless block.call
  end
end

# Represents a Test Spec.
class Spec
  attr_reader :type
  attr_reader :name
  attr_reader :block
  attr_reader :elems

  # @param type [Symbol] Either `:describe`, `:test`, `:before_all`, `:before_each`, `:after_each` or `:after_all`.
  def initialize(type, opts = {})
    @type = type
    @name = opts.fetch(:name, nil)
    @block = opts.fetch(:block, nil)
    @elems = []
  end

  def fetch_elems(type)
    @elems.find_all { |e| e.type == type }
  end

  def describe(name, &block)
    new_elem = Spec.new(:describe, :name => name)
    new_elem.instance_eval &block
    @elems << new_elem
  end

  def it(name, &block)
    @elems << Spec.new(:test, :name => name, :block => block)
  end

  def before_all(&block)
    @elems << Spec.new(:before_all, :block => block)
  end

  def before_each(&block)
    @elems << Spec.new(:before_each, :block => block)
  end

  def after_each(&block)
    @elems << Spec.new(:after_each, :block => block)
  end

  def after_all(&block)
    @elems << Spec.new(:after_all, :block => block)
  end
end

# Represents a Spec Runner.
class SpecRunner
  def initialize(engine, spec)
    @engine = engine
    @spec = spec
  end

  def run
    raise 'wtf' if @spec.type != :describe

    puts "  #{@spec.name}" if @spec.name

    report = Report.new
    @spec.fetch_elems(:describe).each do |sub_desc|
      new_report = SpecRunner.new(@engine, sub_desc).run
      report = report.merge(new_report)
    end

    tests = @spec.fetch_elems(:test)
    return report if tests.empty?

    ctx = Test.new(@engine)
    @spec.fetch_elems(:before_all).each do |sub_task|
      ctx.instance_eval &sub_task.block
    end
    tests.each do |test|
      @spec.fetch_elems(:before_each).each do |sub_task|
        ctx.instance_eval &sub_task.block
      end
      catch :failed do
        begin
          ctx.instance_eval &test.block
        rescue => e
          puts "    [ko] #{test.name}"
          report.add_test_failed
          throw :failed
        end

        puts "    [ok] #{test.name}"
        report.add_test_passed
      end
      @spec.fetch_elems(:after_each).each do |sub_task|
        ctx.instance_eval &sub_task.block
      end
    end
    @spec.fetch_elems(:after_all).each do |sub_task|
      ctx.instance_eval &sub_task.block
    end

    report
  end
end

# Represents a Test Engine.
class Engine
  attr_reader :program
  attr_reader :spec_dir

  def initialize(opts = {})
    @program = opts.fetch(:program, nil)
    @spec_dir = opts.fetch(:spec_dir, nil)
  end

  def set_program(prog)
    @program = prog
  end

  def set_spec_dir(dir)
    @spec_dir = dir
  end

  # @return [Report]
  def run_spec(filename)
    puts File.basename(filename)
    spec = Spec.new(:describe)
    spec.instance_eval File.read(filename)
    SpecRunner.new(self, spec).run
  end

  # Run test specs.
  def run
    raise 'missing spec_dir' unless @spec_dir

    report = Report.new
    Dir.entries(@spec_dir).sort.each do |entry_name|
      unless entry_name =~ /^.+_spec[.]rb$/
        next
      end

      new_report = run_spec("#{@spec_dir}/#{entry_name}")
      report = report.merge(new_report)
      puts ''
    end

    puts "Test Summary:"
    puts "  Total:  #{report.test_total_cnt}"
    puts "  Passed: #{report.test_passed_cnt}"
    puts "  Failed: #{report.test_failed_cnt}"
  end
end
