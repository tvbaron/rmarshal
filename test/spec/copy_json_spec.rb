require_relative '../app_helper'

describe 'copy:json' do
  context = AppHelper.new_context('copy_json')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'copy JSON file to JSON file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies (default)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect01.json"))
    end

    it 'copies (pretty)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'copies (eol)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "--eol", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect03.json"))
    end

    it 'copies (pretty, eol)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "--pretty", "--eol", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect04.json"))
    end
  end

  describe 'copy JSON file to JSON stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies (default)' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect01.json"))
    end

    it 'copies (pretty)' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "--pretty", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'copies (eol)' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "--eol", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect03.json"))
    end

    it 'copies (pretty, eol)' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--copy", "--json", "--pretty", "--eol", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect04.json"))
    end
  end

  describe 'copy JSON stdin to JSON file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies (default)' do
      sb = StringIO.new
      sb.puts '{"name":"Althea"}'
      AppHelper.exec_prog ["--json", "-", "--copy", "#{AppHelper.tmpdir}/out.json"], :stdin => sb
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect01.json"))
    end

    it 'copies (pretty)' do
      sb = StringIO.new
      sb.puts '{"name":"Althea"}'
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"], :stdin => sb
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'copies (eol)' do
      sb = StringIO.new
      sb.puts '{"name":"Althea"}'
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "--eol", "#{AppHelper.tmpdir}/out.json"], :stdin => sb
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect03.json"))
    end

    it 'copies (pretty, eol)' do
      sb = StringIO.new
      sb.puts '{"name":"Althea"}'
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "--pretty", "--eol", "#{AppHelper.tmpdir}/out.json"], :stdin => sb
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect04.json"))
    end
  end

  describe 'copy JSON stdin to JSON stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'copies (default)' do
      si = StringIO.new
      si.puts '{"name":"Althea"}'
      so = StringIO.new
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "-"], :stdin => si, :stdout => so
      expect(so.string).to eq(File.read("#{context.datadir}/expect01.json"))
    end

    it 'copies (pretty)' do
      si = StringIO.new
      si.puts '{"name":"Althea"}'
      so = StringIO.new
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "--pretty", "-"], :stdin => si, :stdout => so
      expect(so.string).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'copies (eol)' do
      si = StringIO.new
      si.puts '{"name":"Althea"}'
      so = StringIO.new
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "--eol", "-"], :stdin => si, :stdout => so
      expect(so.string).to eq(File.read("#{context.datadir}/expect03.json"))
    end

    it 'copies (pretty, eol)' do
      si = StringIO.new
      si.puts '{"name":"Althea"}'
      so = StringIO.new
      AppHelper.exec_prog ["--json", "-", "--copy", "--json", "--pretty", "--eol", "-"], :stdin => si, :stdout => so
      expect(so.string).to eq(File.read("#{context.datadir}/expect04.json"))
    end
  end
end
