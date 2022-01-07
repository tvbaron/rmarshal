require_relative '../app_helper'

describe 'transform' do
  context = AppHelper.new_context('transform')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'produce 1 object to a JSON file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'produces' do
      AppHelper.exec_prog ["--transform", "#{context.datadir}/script01.lua", "--json", "--pretty", "--eol", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect01.json"))
    end
  end

  describe 'produce 2 objects to 2 JSON files' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'produces' do
      AppHelper.exec_prog ["--transform", "#{context.datadir}/script02.lua", "--json", "--pretty", "--eol", "#{AppHelper.tmpdir}/out1.json", "--json", "--pretty", "--eol", "#{AppHelper.tmpdir}/out2.json"]
      expect(File.read("#{AppHelper.tmpdir}/out1.json")).to eq(File.read("#{context.datadir}/expect01.json"))
      expect(File.read("#{AppHelper.tmpdir}/out2.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end
  end
end
