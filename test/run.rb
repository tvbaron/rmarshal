#!/usr/bin/env ruby
# frozen_string_literal: true

require 'fileutils'
require 'open3'
require 'stringio'

require_relative 'engine'

basedir = File.dirname(File.absolute_path(__FILE__))
topdir = File.dirname(basedir)

engine = Engine.new
engine.set_program "#{topdir}/target/debug/rmarshal"
engine.set_spec_dir "#{basedir}/spec"
engine.run
