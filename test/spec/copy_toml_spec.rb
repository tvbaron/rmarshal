require_relative '../app_helper'

describe 'copy:toml' do
  context = AppHelper.new_context('copy_toml')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'copy TOML file to TOML file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      AppHelper.exec_prog ["#{context.datadir}/input01.toml", "--copy", "#{AppHelper.tmpdir}/out.toml"]
      expect(File.read("#{AppHelper.tmpdir}/out.toml")).to eq(File.read("#{context.datadir}/expect01.toml"))
    end
  end

  describe 'copy TOML file to TOML stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.toml", "--copy", "--toml", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect01.toml"))
    end
  end

  describe 'copy TOML stdin to TOML file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      sb = StringIO.new
      sb.puts 'name = "Althea"'
      AppHelper.exec_prog ["--toml", "-", "--copy", "#{AppHelper.tmpdir}/out.toml"], :stdin => sb
      expect(File.read("#{AppHelper.tmpdir}/out.toml")).to eq(File.read("#{context.datadir}/expect01.toml"))
    end
  end

  describe 'copy TOML stdin to TOML stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      si = StringIO.new
      si.puts 'name = "Althea"'
      so = StringIO.new
      AppHelper.exec_prog ["--toml", "-", "--copy", "--toml", "-"], :stdin => si, :stdout => so
      expect(so.string).to eq(File.read("#{context.datadir}/expect01.toml"))
    end
  end
end
