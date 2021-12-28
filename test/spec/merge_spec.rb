require_relative '../app_helper'

describe 'merge' do
  context = AppHelper.new_context('merge')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'merge to JSON file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'merges' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "#{context.datadir}/input02.yaml", "--merge", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect01.json"))
    end
  end
end
