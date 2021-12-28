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

    it 'copies (default)' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "--copy", "#{AppHelper.tmpdir}/out.yaml"]
      expect(File.read("#{AppHelper.tmpdir}/out.yaml")).to eq(File.read("#{context.datadir}/expect01.yaml"))
    end
  end
end
