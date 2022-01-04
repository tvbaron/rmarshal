require_relative '../app_helper'

describe 'concat' do
  context = AppHelper.new_context('concat')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'concat to JSON file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'concatenates (1)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "--concat", "--json", "--eol", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect01.json"))
    end

    it 'concatenates (2)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "#{context.datadir}/input02.yaml", "--concat", "--json", "--eol", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'concatenates (3)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "#{context.datadir}/input02.yaml", "#{context.datadir}/input03.yaml", "--concat", "--json", "--eol", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect03.json"))
    end
  end
end
