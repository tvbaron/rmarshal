require_relative '../app_helper'

describe 'copy:yaml' do
  context = AppHelper.new_context('copy_yaml')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'copy YAML file to YAML file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "--copy", "#{AppHelper.tmpdir}/out.yaml"]
      expect(File.read("#{AppHelper.tmpdir}/out.yaml")).to eq(File.read("#{context.datadir}/expect01.yaml"))
    end
  end

  describe 'copy YAML file to YAML stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "--copy", "--yaml", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect01.yaml"))
    end
  end

  describe 'copy YAML stdin to YAML file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      sb = StringIO.new
      sb.puts <<EOS
---
name: Althea
EOS
      AppHelper.exec_prog ["--yaml", "-", "--copy", "#{AppHelper.tmpdir}/out.yaml"], :stdin => sb
      expect(File.read("#{AppHelper.tmpdir}/out.yaml")).to eq(File.read("#{context.datadir}/expect01.yaml"))
    end
  end

  describe 'copy YAML stdin to YAML stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies' do
      si = StringIO.new
      si.puts <<EOS
---
name: Althea
EOS
      so = StringIO.new
      AppHelper.exec_prog ["--yaml", "-", "--copy", "--yaml", "-"], :stdin => si, :stdout => so
      expect(so.string).to eq(File.read("#{context.datadir}/expect01.yaml"))
    end
  end
end
